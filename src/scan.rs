use crate::peaks::Peak;
use itertools::Itertools;

pub struct PeakScan {
    data: Box<[Peak]>,
}

impl PeakScan {
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

    pub fn peaks(&self) -> &[Peak] {
        &self.data
    }

    pub fn resolution(&self) -> usize {
        self.data.len()
    }
}
