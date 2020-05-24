use rand;
use rand::RngCore;
use std::collections::HashMap;

/// Computes the mean, median, and mode of a random sequence of integers.
fn main() {
    let mut numbers = generate_integers(4096);
    numbers.sort_unstable();
    println!("Generated 4096 integers.");

    let mean = mean(&numbers);
    let median = median(&numbers);
    let mode = mode(&numbers).unwrap(); // cannot be None

    println!("Mean: {}", mean);
    println!("Median: {}", median);
    println!("Mode: {}", mode);
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
/// 
/// Examples:
/// ```
/// let seq = vec![1, 2, 3];
/// assert_eq!(mean(seq), 2.0);
/// ```
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

/// Returns the mode of a sequence of integers, if one exists.
/// If there are multiple modes, it may return any of them.
/// Returns `None` if the input is empty.
fn mode(ints: &Vec<u32>) -> Option<u32> {
    let mut counts: HashMap<u32, u32> = HashMap::new();
    for x in ints {
        if let Some(count) = counts.get_mut(x) {
            *count += 1;
        } else {
            counts.insert(*x, 1);
        }
    }
    counts.iter().max_by_key(|(_val, count)| *count)
                 .and_then(|(val, _count)| Some(*val))
}
