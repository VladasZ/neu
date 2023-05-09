use std::{fmt::Debug, ops::Range};

use rtools::{sleep, Random};

trait Species<In, Out>: Random + Copy + Debug {
    fn live(&self, inp: In) -> Out;
    fn error(&self) -> f32;
    fn set_error(&mut self, err: f32);
    fn mutate(&mut self, val: f32);
}

#[derive(Clone, Copy, Debug)]
struct Doubler {
    pub mult:  f32,
    pub error: f32,
}

impl Species<f32, f32> for Doubler {
    fn live(&self, inp: f32) -> f32 {
        inp * self.mult
    }

    fn error(&self) -> f32 {
        self.error
    }

    fn set_error(&mut self, err: f32) {
        self.error = err;
    }

    fn mutate(&mut self, val: f32) {
        self.mult += val;
    }
}

impl Random for Doubler {
    fn random() -> Self {
        Self {
            mult:  f32::random_in(-100.0..100.0),
            error: f32::MAX,
        }
    }
}

#[derive(Debug)]
struct Selection<S: Species<f32, f32>, const GENERATION_SIZE: usize> {
    generation: usize,
    species:    [S; GENERATION_SIZE],
    bestest:    S,
}

impl<S: Species<f32, f32>, const GENERATION_SIZE: usize> Selection<S, GENERATION_SIZE> {
    fn new() -> Self {
        let mut species = [S::random(); GENERATION_SIZE];
        for sp in species.iter_mut().take(GENERATION_SIZE) {
            *sp = S::random()
        }
        Self {
            generation: 0,
            species,
            bestest: S::random(),
        }
    }

    fn trial_species(species: &mut S, expected: &[(f32, f32)]) {
        let mut error = f32::default();
        for trial in expected {
            let result = species.live(trial.0);
            error += (trial.1 - result).abs();
        }
        species.set_error(error)
    }

    fn generation(&mut self, expected: &[(f32, f32)]) {
        for sp in &mut self.species {
            Self::trial_species(sp, expected);
        }
        self.generation += 1;
    }

    fn selection(&mut self) {
        self.species.sort_by(|a, b| a.error().total_cmp(&b.error()));

        if self.species[0].error() < self.bestest.error() {
            self.bestest = self.species[0];
        }

        for i in GENERATION_SIZE / 2..GENERATION_SIZE {
            self.species[i] = self.species[i - GENERATION_SIZE / 2];
        }
    }

    fn mutate(&mut self, rg: Range<f32>) {
        for sp in &mut self.species {
            sp.mutate(f32::random_in(rg.clone()));
        }
    }

    fn total_error(&self) -> f32 {
        dbg!(self.bestest.error());
        self.species.iter().map(|s| s.error()).sum()
    }

    fn apex(&self) -> Option<&S> {
        self.species.iter().find(|s| s.error() == 0.0)
    }
}

fn main() {
    let mut sel = Selection::<Doubler, 20>::new();
    dbg!(&sel);

    let mut apex = Doubler::random();

    loop {
        sel.generation(&[(2.0, 4.0), (5.0, 10.0), (100.0, 200.0)]);
        sel.selection();
        sel.mutate(-0.0001..0.0001);

        if let Some(ap) = sel.apex() {
            apex = *ap;
            break;
        }
    }

    dbg!(&sel);

    dbg!(&apex);
}
