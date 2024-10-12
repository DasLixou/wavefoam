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
    /// This will use [`SlicePeakExt::peak_avx2`] if available or fall back to [`SlicePeakExt::peak_naive`].
    /// When you know that your chunks are not larger than 15 elements,
    /// calling [`SlicePeakExt::peak_naive`] directly might be better.
    ///
    /// # Example
    ///
    /// ```ignore
    /// samples.chunks(4).map(SlicePeakExt::peak)
    /// ```
    fn peak(self) -> Peak<Self::Item> {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        if is_x86_feature_detected!("avx2") {
            return Self::peak_avx2(self);
        }
        Self::peak_naive(self)
    }

    /// Returns the highest and lowest elements in this slice.
    ///
    /// The implementation is optimized with AVX2's 8xf32 SIMD instructions,
    /// but only has a real performance gain for chunks of 16 elements or more.
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn peak_avx2(self) -> Peak<Self::Item>;

    /// Returns the highest and lowest elements in this slice.
    ///
    /// The implementation is naive and not optimized,
    /// but can still perform better on small chunks when used directly.
    fn peak_naive(self) -> Peak<Self::Item>;
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
