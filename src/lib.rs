use num::complex::{Complex, Complex32};
use std::f32::consts::PI;

pub fn fft(tone: &Vec<f32>) -> Vec<Complex32> {
  let n: i32 = tone.len() as i32;
  if n % 2 == 0 {
    panic!("The number of points in the 'tone' must be equal to the power of 2");
  }
  let mut n_bits: i32 = 0;

  while (1 << n_bits) < n {
    n_bits += 1;
  }

  let mut reverse_positions: Vec<i32> = vec![0; n as usize];
  let mut high_bit: i32 = -1;
  for i in 1..n as usize {
    if i & (i - 1) == 0 {
      high_bit += 1;
    }
    reverse_positions[i] = reverse_positions[i ^ (1 << high_bit)];
    reverse_positions[i] |= 1 << (n_bits - high_bit - 1);
  }

  let mut precalc_roots: Vec<Complex32> = Vec::new();
  for i in 0..n {
    let alpha: f32 = 2.0 * PI * i as f32 / n as f32;
    precalc_roots.push(Complex::new(alpha.cos(), alpha.sin()));
  }

  let mut current_tone: Vec<Complex32> = Vec::new();
  for i in 0..n as usize {
    current_tone.push(Complex::new(tone[reverse_positions[i] as usize], 0.0));
  }

  let mut block_len: i32 = 1;
  while block_len < n {
    let mut new_tone: Vec<Complex32> = vec![Complex::new(0.0, 0.0); n as usize];
    let block_step: i32 = n / (2 * block_len);
    let mut new_tone_i: i32 = 0;
    while new_tone_i < n {
      for i in 0..block_len {
        let temp_val: Complex32 = precalc_roots[(i * block_step) as usize]
          * current_tone[(new_tone_i + block_len) as usize];
        new_tone[new_tone_i as usize] = current_tone[new_tone_i as usize] + temp_val;
        new_tone[(new_tone_i + block_len) as usize] = current_tone[new_tone_i as usize] - temp_val;
        new_tone_i += 1;
      }
      new_tone_i += block_len;
    }
    current_tone = new_tone;
    block_len <<= 1;
  }

  current_tone
}

pub fn fft_freq(n: i32, sample_rate: &i32) -> Vec<f32> {
  let temp_value: f32 = *sample_rate as f32 / n as f32;
  let mut frequencies: Vec<f32> = Vec::new();
  for i in 0..n {
    frequencies.push(i as f32 * temp_value);
  }

  frequencies
}

pub fn normalize_complex_vector(mas: &Vec<Complex32>) -> Vec<f32> {
  let mut result: Vec<f32> = Vec::new();
  for &ob in mas.iter() {
    result.push(ob.norm());
  }
  result
}

pub fn generate_sin_wave(freq: &f32, sample_rate: &i32, duration: &f32) -> Vec<f32> {
  let n: i32 = (*sample_rate as f32 * duration) as i32;
  let mut wave: Vec<f32> = Vec::new();
  let loop_step: f32 = 1.0 / *sample_rate as f32;

  for i in 0..n {
    wave.push((loop_step * i as f32 * freq * PI * 2.0).sin());
  }

  wave
}
