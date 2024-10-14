use super::{Peak, PeakExt};
use crate::utils::IterExt as _;

impl PeakExt for f32 {
    fn peak(iter: impl Iterator<Item = Self>) -> Option<Peak<Self>> {
        iter.reduce_with(
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
