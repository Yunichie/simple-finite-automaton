use std::collections::{HashMap, HashSet};

pub struct DFA {
    q: HashSet<u32>,
    sigma: String,
    delta: HashMap<(u32, char), u32>,
    q0: u32,
    f: HashSet<u32>,
}

impl DFA {
    pub fn new(sigma: String) -> Self {
        DFA {
            q: HashSet::from([0, 1, 2]),
            sigma,
            delta: HashMap::from([
                ((0, 'a'), 0),
                ((0, 'b'), 1),
                ((1, 'a'), 2),
                ((1, 'b'), 1),
                ((2, 'a'), 2),
                ((2, 'b'), 2),
            ]),
            q0: 0,
            f: HashSet::from([0, 1]),
        }
    }

    pub fn run(self) -> bool {
        let mut q = self.q0;

        for val in self.sigma.chars() {
            if let Some(state) = self.delta.get(&(q, val)) {
                q = *state;
            }
        }

        self.f.contains(&q)
    }
}
