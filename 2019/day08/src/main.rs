// Chunk the image string into vectors of 150 (25 x 6) "pixels".
// Create a Layer object which runs through the vector counting
// 0's, 1's and 2's.
// Creating the image is just stacking the layers and taking the
// value of the first non-transparent pixel in a given location.
// Then "draw" the image by writing out the vector in rows and
// converting values to '*' and ' ' to try and make the letters
// visible.

use std::cmp::{Ordering, PartialOrd};

fn main() {
  let layers = include_str!("./image.txt")
    .as_bytes()
    .chunks(25 * 6)
    .enumerate()
    .map(|(i, pixels)| Layer::new(i, pixels))
    .collect::<Vec<Layer>>();

  let min_zeros = layers.iter().min().unwrap();
  println!("{:?} {}", min_zeros, min_zeros.digits());

  let image = layers[0].clone();
  let image = layers.iter().skip(1).fold(image, |i, l| i.stack(l));
  image.draw();

  // GKCKH
}

#[derive(Clone, Debug, Eq)]
struct Layer {
  pos: usize,
  ones: usize,
  twos: usize,
  zeros: usize,
  pixels: Vec<u8>
}

impl Layer {
  pub fn new(pos: usize, chunk: &[u8]) -> Layer {
    let mut ones = 0;
    let mut twos = 0;
    let mut zeros = 0;
    let pixels: Vec<u8> = chunk.iter().map(|p| {
      let v = p - 48;
      if v == 1 { ones += 1 }
      else if v == 2 { twos += 1 }
      else { zeros += 1 }
      v
    }).collect();

    Layer { pos, ones, twos, zeros, pixels }
  }

  pub fn digits(&self) -> usize {
    self.ones * self.twos
  }

  pub fn draw(&self) {
    self.pixels.chunks(25)
      .map(|l| l.iter().map(|v| if *v == 0u8 { '*' } else { ' ' }).collect())
      .for_each(|l:String| println!("{:?}", l));
  }

  pub fn stack(&self, other: &Self) -> Self {
    let pixels = self.pixels.iter().enumerate().map(|(i, &p)| {
      if p == 2 {
        other.pixels[i]
      } else {
        p
      }
    })
    .collect();

    Layer { pos: 0, ones: 0, twos: 0, zeros: 0, pixels }
  }
}

impl Ord for Layer {
  fn cmp(&self, other: &Self) -> Ordering {
    self.zeros.cmp(&other.zeros)
  }
}

impl PartialEq for Layer {
  fn eq(&self, other: &Self) -> bool {
    self.zeros.eq(&other.zeros)
  }
}

impl PartialOrd for Layer {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.zeros.partial_cmp(&other.zeros)
  }
}