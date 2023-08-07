use super::*;

#[derive(Clone, Debug)]
pub struct EnhanceMap<T: Ord> {
    pub mapping: BTreeMap<u16, EnhanceLevel<T>>,
}

#[derive(Clone, Debug)]
pub struct EnhanceMatrix {
    pub matrix: DMatrix<f64>,
    pub breakable: bool,
}

impl<T: Ord> Default for EnhanceMap<T> {
    fn default() -> Self {
        Self { mapping: BTreeMap::new() }
    }
}

impl<T: Ord> EnhanceMap<T> {
    pub fn breakable(&self) -> bool {
        self.mapping.values().any(|level| level.broken_rate > 0.0)
    }

    pub fn max_level(&self) -> u16 {
        self.mapping.iter().map(|(level, data)| data.max_level(*level)).max().unwrap_or(0)
    }

    pub fn as_matrix(&self) -> EnhanceMatrix {
        if self.breakable() {
            unimplemented!()
        }
        else {
            let max = self.max_level() as usize;
            let mut matrix = DMatrix::from_element(max + 1, max + 1, 0.0);
            for (base, data) in &self.mapping {
                for (level, rate) in data.as_absolute(*base).absolute_rate {
                    matrix[(*base as usize, level as usize)] = rate;
                }
            }
            matrix[(max, max)] = 1.0;
            EnhanceMatrix { matrix, breakable: false }
        }
    }
}
