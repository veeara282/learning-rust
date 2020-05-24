use rand;
use rand::RngCore;

/// Computes the mean, median, and mode of a random sequence of integers.
fn main() {
    let mut numbers = generate_integers(4096);
    numbers.sort_unstable();

    let mean = mean(&numbers);
    let median = median(&numbers);

    println!("Hello, world!");
}

/// Generates `n` integers between 0 and 255.
fn generate_integers(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut output = Vec::with_capacity(n);
    for _i in 0..n {
        output.push(rng.next_u32() % 256);
    }
    output
}

/// Computes the mean of a sequence of integers.
fn mean(ints: &Vec<u32>) -> f32 {
    let mut sum: f32 = 0.0;
    for x in ints {
        sum += *x as f32;
    }
    let n = ints.len() as f32;
    sum / n
}

/// Computes the median of a sorted sequence of integers.
fn median(ints: &Vec<u32>) -> f32 {
    let n = ints.len();
    if n % 2 == 1 {
        ints[n / 2] as f32
    } else {
        let m1 = ints[n / 2 - 1] as f32;
        let m2 = ints[n / 2] as f32;
        (m1 + m2) / 2.0
    }
}
