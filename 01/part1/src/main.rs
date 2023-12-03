use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/output.txt")?;
    let reader = BufReader::new(file);
    let mut total_sum = 0;

    for line in reader.lines() {
        let line = line?;
        let mut first_digit = None;
        let mut last_digit = None;

        for c in line.chars() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap();
                if first_digit.is_none() {
                    first_digit = Some(digit);
                }
                last_digit = Some(digit);
            }
        }

        let sum = match (first_digit, last_digit) {
            (Some(first), Some(last)) if first == last => first * 11,
            (Some(first), Some(last)) => 10 * first + last,
            _ => 0,
        };
        total_sum += sum;
    }
    println!("Somme totale: {}", total_sum);
    Ok(())
}
