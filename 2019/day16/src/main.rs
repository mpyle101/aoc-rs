// Part one is brute force with a few optimizations from chunking
// past the zero runs.
// Part 2 became almost trivial after a few key realizations.
// A) you don't have to solve the general problem, just your
// specific problem which means finding the 8 digits at the
// specific offset.
// B) it's really fast to calculate the second half of the
// phase because the pattern will only be all zeros followed
// by all ones so the values are accumlated sums from the end
// toward the front modulo ten.
// C) the signal offset is 5,979,013 so we really only need
// to calculate that offset and beyond (that means only 600K
// out of the 6.5M) which puts us firmly in the second half
// of the phase so we never have to bother calculating the
// rest of the phase values.
// Note: the realizations took a little bit.

fn main() {
  let signal = load(include_str!("./signal.txt"));
  let result = fft(100, &signal);
  println!("FFT: {}", result);

  let result = part_two(&signal);
  println!("FFT: {}", result);
}

fn load(signal: &str) -> Vec<i32> {
  const RADIX: u32 = 10;
  signal.chars()
    .map(|c| c.to_digit(RADIX).unwrap() as i32)
    .collect()
}

fn concat(digits: &[i32]) -> i32 {
  digits.iter().fold(0, |acc, v| acc * 10 + v)
}

fn fft(phases: i32, signal: &[i32]) -> String {
  let to_char = |v: &i32| (*v as u8 + b'0') as char;
  let digits: Vec<_> = (0..phases).fold(signal.into(), |s, _| fft_phase(&s));
  digits.iter().take(8).map(to_char).collect()
}

fn fft_phase(signal: &[i32]) -> Vec<i32> {
  signal.iter()
    .enumerate()
    .map(|(i, _)| fft_element(signal, i))
    .collect()
}

fn fft_element(signal: &[i32], el: usize) -> i32 {
  let mut sign = -1;
  signal[el..]
    .chunks(el + 1)
    .step_by(2)
    .map(|vec| {
      sign *= -1;
      vec.iter().sum::<i32>() * sign
    })
    .sum::<i32>()
    .abs() %10
}

fn part_two(pattern: &[i32]) -> i32 {
  let offset = concat(&pattern[0..7]) as usize;
  let mut signal: Vec<_> = pattern.iter().cycle()
    .take(pattern.len() * 10000).copied().collect();

  let mut phase = vec![0; signal.len() - offset];
  for _ in 0..100 {
    signal.iter().rev().zip(phase.iter_mut().rev())
      .fold(0, |acc, t| { *t.1 = (acc + t.0) % 10; acc + t.0 });
    signal = phase.clone();
  }

  concat(&phase[0..8])
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let signal = load(include_str!("./signal.txt"));
    let result = fft(100, &signal);
    assert_eq!(result, "40580215");

    let result = part_two(&signal);
    assert_eq!(result, 22621597);
  }

  #[test]
  fn four_phases() {
    let signal = vec![1,2,3,4,5,6,7,8];

    let signal = fft_phase(&signal);
    assert_eq!(&signal, &[4,8,2,2,6,1,5,8]);
    
    let signal = fft_phase(&signal);
    assert_eq!(&signal, &[3,4,0,4,0,4,3,8]);
    
    let signal = fft_phase(&signal);
    assert_eq!(&signal, &[0,3,4,1,5,5,1,8]);
    
    let signal = fft_phase(&signal);
    assert_eq!(&signal, &[0,1,0,2,9,4,9,8]);
  }

  #[test]
  fn run_four_phases() {
    let signal = vec![1,2,3,4,5,6,7,8];
    let result = fft(4, &signal);

    assert_eq!(result, "01029498");
  }

  #[test]
  fn part_two_works() {
    let signal = load("03036732577212944063491565474664");
    let result = part_two(&signal);
    assert_eq!(result, 84462026);

    let signal = load("02935109699940807407585447034323");
    let result = part_two(&signal);
    assert_eq!(result, 78725270);

    let signal = load("03081770884921959731165446850517");
    let result = part_two(&signal);
    assert_eq!(result, 53553731);
  }
}