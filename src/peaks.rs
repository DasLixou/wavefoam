#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Peak<T> {
    pub min: T,
    pub max: T,
}

pub trait SlicePeakExt {
    type Item;

    fn peak(self) -> Peak<Self::Item>;
}

impl<T> SlicePeakExt for &[T]
where
    T: Peakable,
{
    type Item = T;

    /// Returns the highest and lowest elements in this slice according to [`Peakable`]
    ///
    /// # Example
    ///
    /// ```ignore
    /// samples.chunks(4).map(SlicePeakExt::peak)
    /// ```
    fn peak(self) -> Peak<Self::Item> {
        self.into_iter()
            .fold(Peak::default(), |peak, &sample| Peak {
                min: sample.lower(peak.min),
                max: sample.higher(peak.max),
            })
    }
}

pub trait Peakable: Default + Copy {
    fn lower(self, other: Self) -> Self;
    fn higher(self, other: Self) -> Self;
}

impl Peakable for f32 {
    fn lower(self, other: Self) -> Self {
        self.min(other)
    }
    fn higher(self, other: Self) -> Self {
        self.max(other)
    }
}

#[cfg(test)]
mod test {
    use crate::peaks::{Peak, SlicePeakExt};

    #[test]
    fn peaks_4() {
        let samples: &[f32] = &[
            0.0, 1.4, -3.4, 2.1, // chunk 1
            -1.3, 5.3, 2.1, 0.9, // chunk 2
        ];
        let peaks: Vec<_> = samples.chunks(4).map(SlicePeakExt::peak).collect();

        assert_eq!(
            peaks,
            &[
                Peak {
                    min: -3.4,
                    max: 2.1
                },
                Peak {
                    min: -1.3,
                    max: 5.3
                }
            ]
        );
    }
}
