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
