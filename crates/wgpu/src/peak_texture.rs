use bikeshedwaveform_core::peaks::PeakExt;

pub struct PeakTexture {}

impl PeakTexture {
    pub fn from_slice<T>(slice: &[T], chunk_size: usize) -> Self
    where
        [T]: PeakExt,
    {
        todo!()
    }
}
