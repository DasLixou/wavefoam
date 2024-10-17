use bikeshedwaveform_core::peaks::Peak;
use itertools::Itertools;
use wgpu::{Extent3d, Queue, Texture, TextureDescriptor, TextureFormat, TextureUsages};

pub struct PeakTexture {
    data: Box<[Peak]>,
}

impl PeakTexture {
    const FORMAT: TextureFormat = TextureFormat::Rg32Float; // TODO: do we just want 16-bit? Should user be able to decide?

    pub fn from_iter(iter: impl Itertools<Item = f32>, chunk_size: usize) -> Self {
        Self::from_chunks(
            iter.chunks(chunk_size)
                .into_iter()
                .map(|chunk| Peak::from_iter(chunk).unwrap_or_default()),
        )
    }

    pub fn from_chunks(iter: impl Iterator<Item = Peak>) -> Self {
        Self {
            data: iter.collect(),
        }
    }

    pub fn texture_size(&self) -> Extent3d {
        Extent3d {
            width: self.data.len() as u32,
            height: 1,
            depth_or_array_layers: 1,
        }
    }

    pub fn texture_descriptor(&self) -> TextureDescriptor {
        TextureDescriptor {
            label: Some("peak texture"),
            size: self.texture_size(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D1,
            format: Self::FORMAT,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        }
    }

    pub fn queue_texture_write(&self, queue: &Queue, texture: &Texture) {
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(&self.data),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(8 * self.data.len() as u32),
                rows_per_image: None,
            },
            self.texture_size(),
        );
    }
}
