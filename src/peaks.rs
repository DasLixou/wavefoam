pub mod f32;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Peak<T> {
    pub min: T,
    pub max: T,
}

pub trait SlicePeakExt: Sized {
    type Item;

    /// Returns the highest and lowest elements in this slice.
    ///
    /// # Example
    ///
    /// ```ignore
    /// samples.chunks(4).map(SlicePeakExt::peak)
    /// ```
    fn peak(self) -> Peak<Self::Item>;
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

    #[test]
    fn peak_8() {
        let samples: &[f32] = &[
            0.0, 1.4, -3.4, 2.1, // chunk 1
            -1.3, 5.3, 2.1, 0.9, // chunk 2
        ];
        let peak = SlicePeakExt::peak(samples);

        assert_eq!(
            peak,
            Peak {
                min: -3.4,
                max: 5.3
            }
        );
    }
}
