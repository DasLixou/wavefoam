use super::{Peak, SlicePeakExt};

impl SlicePeakExt for &[f32] {
    type Item = f32;

    fn peak(self) -> Peak<Self::Item> {
        // TODO: think about default - when it's between 2. and 5., min would still be 0
        self.into_iter()
            .fold(Peak::default(), |peak, &sample| Peak {
                min: sample.min(peak.min),
                max: sample.max(peak.max),
            })
    }
}
