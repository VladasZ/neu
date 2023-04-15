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
    // generation: usize,
    species: [S; GENERATION_SIZE],
    bestest: S,
}

impl<S: Species<f32, f32>, const GENERATION_SIZE: usize> Selection<S, GENERATION_SIZE> {
    fn new() -> Self {
        let mut species = [S::random(); GENERATION_SIZE];
        for sp in species.iter_mut().take(GENERATION_SIZE) {
            *sp = S::random()
        }
        Self {
            // generation: 0,
            species,
            bestest: S::random(),
        }
    }

    fn generation(&mut self, input: f32, output: f32) {
        for sp in &mut self.species {
            let result = sp.live(input);
            let error = (output - result).abs();
            sp.set_error(error);
        }
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
}

fn main() {
    let mut sel = Selection::<Doubler, 20>::new();
    dbg!(&sel);

    loop {
        sel.generation(2.0, 4.0);
        sel.selection();
        sel.mutate(-0.001..0.001);
        dbg!(sel.total_error());
        sleep(0.01);
    }
}
