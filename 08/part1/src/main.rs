use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Unable to read file");
    let split = input.find('\n').unwrap();

    let mut map = vec![0u32; 0b11001_11001_11001 + 1];
    let encode = |node: &str| -> u32 {
        ((node.chars().nth(0).unwrap_or('A') as u32 - 'A' as u32) << 10)
            | ((node.chars().nth(1).unwrap_or('A') as u32 - 'A' as u32) << 5)
            | (node.chars().nth(2).unwrap_or('A') as u32 - 'A' as u32)
    };

    input[split + 1..]
        .lines()
        .for_each(|line| {
            if line.len() >= 15 {
                let source_node = &line[0..3];
                let destination_node_1 = &line[7..10];
                let destination_node_2 = &line[12..15];
                map[encode(source_node) as usize] = encode(destination_node_1) | encode(destination_node_2) << 16;
            }
        });

    let steps = input[0..split].chars();
    let mut current_node = encode("AAA");
    let mut step_count = 0;

    for step in steps.cycle() {
        if step_count >= input.len() {
            break;
        }

        step_count += 1;
        current_node = if step == 'L' {
            map[current_node as usize] & u16::MAX as u32
        } else {
            map[current_node as usize] >> 16
        };

        if current_node & 0b11111 == ('Z' as u32 - 'A' as u32) {
            break;
        }
    }

    println!("{}", step_count);
}
