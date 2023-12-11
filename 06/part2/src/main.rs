fn main() {
   
    let input = include_str!("./input.txt");

    
    let mut parts = input.split('\n');

   
    let first_line = parts.next().unwrap();
    let time = parse_line_to_number(&first_line);

   
    let second_line = parts.next().unwrap();
    let dist = parse_line_to_number(&second_line);

    
    let result = calculate_result(time, dist);

    println!("{}", result);
}

fn parse_line_to_number(line: &str) -> usize {
    
    let filtered_chars: Vec<char> = line.chars().filter(|&c| !c.is_whitespace()).collect();
    let parsed_number: usize = filtered_chars.iter().collect::<String>().parse().unwrap();

    parsed_number
}

fn calculate_result(time: usize, dist: usize) -> usize {
    let a = (time - ((time * time - 4 * dist) as f64).sqrt() as usize) / 2;
    let b = time - a;

    let result = b - (b * (time - b) <= dist) as usize - a - (a * (time - a) <= dist) as usize + 1;

    result
}
