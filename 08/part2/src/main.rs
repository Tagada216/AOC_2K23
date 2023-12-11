use std::fs;

fn main() {
    let content = fs::read_to_string("src/input.txt").expect("Failed to read file");
    let lines: Vec<&str> = content.split('\n').collect();

    let steps = lines[0].as_bytes();
    let nodes = &lines[1..];

    let mut map = [0u32; 0b11001_11001_11001 + 1];
    let mut starts = Vec::new();

    fn encode(node: &[u8]) -> u32 {
        ((node[0] - b'A') as u32) << 10 | ((node[1] - b'A') as u32) << 5 | (node[2] - b'A') as u32
    }

    for node in nodes {
        if node.len() >= 15 {
            let enc_node = encode(&node.as_bytes()[0..3]);
            map[enc_node as usize] = encode(&node.as_bytes()[7..10]) | encode(&node.as_bytes()[12..15]) << 16;
            if node.as_bytes()[2] == b'A' {
                starts.push(enc_node);
            }
        }
    }

    // Fonction pour calculer le PGCD
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 { a } else { gcd(b, a % b) }
    }

    // Fonction pour calculer le PPCM
    fn lcm(a: usize, b: usize) -> usize {
        a / gcd(a, b) * b
    }

    let result = starts
        .into_iter()
        .map(|start| {
            steps
                .iter()
                .cycle()
                .scan(start, |state, &step| {
                    *state = if step == b'L' {
                        map[*state as usize] & u16::MAX as u32
                    } else {
                        map[*state as usize] >> 16
                    };
                    Some(*state & 0b11111 == (b'Z' - b'A') as u32)
                })
                .position(|is_end| is_end)
                .unwrap()
                + 1
        })
        .reduce(lcm) // Utilisation de la fonction lcm définie ci-dessus
        .unwrap();

    println!("{}", result);
}
