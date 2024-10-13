use super::{Peak, PeakExt};
use crate::utils::IterExt as _;

impl PeakExt for [f32] {
    type Item = f32;

    fn peak(&self) -> Option<Peak<Self::Item>> {
        self.into_iter().copied().reduce_with(
            |sample| Peak {
                min: sample,
                max: sample,
            },
            |peak, sample| Peak {
                min: sample.min(peak.min),
                max: sample.max(peak.max),
            },
        )
    }
}
