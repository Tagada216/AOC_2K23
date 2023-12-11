use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Unable to read file");

    let parts: Vec<&str> = input.split('\n').collect();
    let first_numbers: Vec<usize> = parts[0]
        .split_whitespace()
        .map(|num_str| num_str.parse().expect("Invalid number"))
        .collect();

    let second_numbers: Vec<usize> = parts[1]
        .split_whitespace()
        .map(|num_str| num_str.parse().expect("Invalid number"))
        .collect();

    let result = first_numbers
        .iter()
        .zip(second_numbers.iter())
        .map(|(&t, &d)| {
            let a = (t - ((t * t - 4 * d) as f64).sqrt() as usize) / 2;
            let b = t - a;
            b - (b * (t - b) <= d) as usize - a - (a * (t - a) <= d) as usize + 1
        })
        .product::<usize>();

    println!("{}", result);
}
