#[derive(Clone)]
pub struct Entry {
    hash: u64,
    score: f64,
}

pub struct TranspositionTable(Vec<Option<Entry>>);

impl TranspositionTable {
    pub fn new(size: usize) -> Self {
        Self(vec![None; size])
    }

    // Always replace
    pub fn set(&mut self, hash: u64, score: f64) {
        let idx = hash as usize % self.0.len();
        self.0[idx] = Some(Entry { hash, score });
    }

    pub fn get(&self, hash: u64) -> Option<f64> {
        let idx = hash as usize % self.0.len();

        self.0[idx]
            .clone()
            .filter(|entry| entry.hash == hash)
            .map(|entry| entry.score)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set() {
        let mut tt = TranspositionTable::new(10);

        tt.set(3, 22.);

        assert_eq!(tt.get(3), Some(22.));
        assert_eq!(tt.get(13), None);
        assert_eq!(tt.get(1), None);
    }
}
