use super::{Peak, PeakExt};
use crate::utils::IterExt as _;

impl PeakExt for f32 {
    fn peak(slice: &[Self]) -> Option<Peak<Self>> {
        slice.into_iter().copied().reduce_with(
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
