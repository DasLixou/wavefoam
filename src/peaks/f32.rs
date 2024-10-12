use std::mem::MaybeUninit;

use super::{Peak, SlicePeakExt};

impl SlicePeakExt for &[f32] {
    type Item = f32;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn peak_avx2(self) -> Peak<Self::Item> {
        #[cfg(target_arch = "x86")]
        use std::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64::*;

        let blocks = self.chunks_exact(8);
        let remainder = blocks.remainder();

        let mut min = unsafe { _mm256_setzero_ps() };
        let mut max = unsafe { _mm256_setzero_ps() };

        for block in blocks {
            let samples = unsafe { _mm256_loadu_ps(block.as_ptr()) };
            min = unsafe { _mm256_min_ps(min, samples) };
            max = unsafe { _mm256_max_ps(max, samples) };
        }

        let min = unsafe {
            let mut array: MaybeUninit<[f32; 8]> = MaybeUninit::uninit();
            // TODO: make array aligned correctly so we can use store instead
            _mm256_storeu_ps(array.as_mut_ptr().cast(), min);
            array.assume_init()
        };
        let max = unsafe {
            let mut array: MaybeUninit<[f32; 8]> = MaybeUninit::uninit();
            _mm256_storeu_ps(array.as_mut_ptr().cast(), max);
            array.assume_init()
        };

        return Peak {
            min: min
                .into_iter()
                .chain(remainder.into_iter().copied())
                .reduce(f32::min)
                .unwrap(),
            max: max
                .into_iter()
                .chain(remainder.into_iter().copied())
                .reduce(f32::max)
                .unwrap(),
        };
    }

    fn peak_naive(self) -> Peak<Self::Item> {
        // TODO: think about default - when it's between 2. and 5., min would still be 0 (and same for avx2 impl)
        self.into_iter()
            .fold(Peak::default(), |peak, &sample| Peak {
                min: sample.min(peak.min),
                max: sample.max(peak.max),
            })
    }
}
