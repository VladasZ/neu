use rtools::Random;

struct Neu<const IN: usize, const OUT: usize> {
    link: f32,
}

impl<const IN: usize, const OUT: usize> Default for Neu<IN, OUT> {
    fn default() -> Self {
        Self { link: f32::random() }
    }
}

impl<const IN: usize, const OUT: usize> Neu<IN, OUT> {
    fn process(&self, input: [f32; IN]) -> [f32; 1] {
        [input[0] * self.link]
    }
}

type Ne = Neu<1, 1>;

struct Selection {}

fn main() {
    println!("Hello, world!");
}
