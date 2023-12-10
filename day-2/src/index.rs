use crate::game::{Game, GameAnalysis};
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Default)]
struct ColorIndex {
    inner: BTreeMap<u64, Vec<u64>>,
}

impl ColorIndex {
    fn insert(&mut self, key: u64, value: u64) {
        self.inner.entry(key).or_insert_with(Vec::new).push(value);
    }

    fn query(&self, query: u64) -> HashSet<u64> {
        self.inner
            .range(..=query)
            .map(|(_, v)| v.iter().copied())
            .flatten()
            .collect::<HashSet<_>>()
    }
}

#[derive(Debug, Default)]
pub struct IndexedGames {
    red: ColorIndex,
    green: ColorIndex,
    blue: ColorIndex,
}

impl IndexedGames {
    pub(crate) fn insert(&mut self, game: Game) {
        let analysis = game.analyze();
        self.red.insert(analysis.max_red, game.id);
        self.green.insert(analysis.max_green, game.id);
        self.blue.insert(analysis.max_blue, game.id);
    }

    pub(crate) fn query(&self, query: GameAnalysis) -> Vec<u64> {
        let red_results = self.red.query(query.max_red);
        let green_results = self.green.query(query.max_green);
        let blue_results = self.blue.query(query.max_blue);

        red_results
            .intersection(&green_results)
            .copied()
            .collect::<HashSet<_>>()
            .intersection(&blue_results)
            .copied()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
    }
}
