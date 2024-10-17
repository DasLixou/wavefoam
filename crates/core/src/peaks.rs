#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
pub struct Peak {
    pub min: f32,
    pub max: f32,
}

impl Default for Peak {
    fn default() -> Self {
        Self { min: 0.0, max: 0.0 }
    }
}

impl Peak {
    pub fn from_iter<I>(iter: I) -> Option<Peak>
    where
        I: IntoIterator<Item = f32>,
    {
        let mut iter = iter.into_iter();
        let first = iter.next()?;
        Some(iter.fold(
            Peak {
                min: first,
                max: first,
            },
            |peak, sample| Peak {
                min: sample.min(peak.min),
                max: sample.max(peak.max),
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::peaks::Peak;

    #[test]
    fn peak_8() {
        let samples: &[f32] = &[0.0, 1.4, -3.4, 2.1, -1.3, 5.3, 2.1, 0.9];
        let peak = Peak::from_iter(samples.iter().copied());

        assert_eq!(
            peak,
            Some(Peak {
                min: -3.4,
                max: 5.3
            })
        );
    }

    #[test]
    fn only_positive() {
        let samples: &[f32] = &[1.4, 2.1, 5.3, 2.1, 0.9];
        let peak = Peak::from_iter(samples.iter().copied());

        assert_eq!(peak, Some(Peak { min: 0.9, max: 5.3 }));
    }
}
