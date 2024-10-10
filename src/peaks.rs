#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Peak {
    pub min: f32,
    pub max: f32,
}

/// Detects peaks in chunks of size `chunk_size` in the data.
///
/// `chunk_size` must be 2 or any power of two above.
///
/// If `chunk_size` does not divide the length of the
/// slice, then the last peak will only work over the remaining data.
pub fn peaks_f32(data: &[f32], chunk_size: usize) -> Vec<Peak> {
    debug_assert!(chunk_size >= 2);
    debug_assert!(chunk_size.is_power_of_two());

    let mut peaks = Vec::with_capacity(data.len().div_ceil(chunk_size));

    for chunk in data.chunks(chunk_size) {
        let peak = chunk
            .iter()
            .fold(Peak { min: 0., max: 0. }, |peak, sample| Peak {
                min: sample.min(peak.min),
                max: sample.max(peak.max),
            });
        peaks.push(peak);
    }

    peaks
}

#[cfg(test)]
mod test {
    use crate::peaks::Peak;

    use super::peaks_f32;

    #[test]
    fn peaks_4() {
        let peaks = peaks_f32(
            &[
                0.0, 1.4, -3.4, 2.1, // chunk 1
                -1.3, 5.3, 2.1, 0.9, // chunk 2
            ],
            4,
        );

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
