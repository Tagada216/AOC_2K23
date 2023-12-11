use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Unable to read file");
    let mut hands = input
        .lines()
        .map(|line| {
            let (mut ranks, mut power) = ([0u8; 13], 0);
            for (i, card_char) in line[..5].chars().enumerate() {
                let card = match card_char {
                    'A' => 12,
                    'K' => 11,
                    'Q' => 10,
                    'J' => 9,
                    'T' => 8,
                    n if n.is_digit(10) => n.to_digit(10).unwrap() as u8 - 2,
                    _ => panic!("Invalid card"),
                };
                ranks[card as usize] += 1;
                power |= (card as u32) << 4 * (4 - i);
            }
            ranks.sort_unstable_by(|a, b| b.cmp(a));
            power |= match ranks[0] {
                5 => 6,
                4 => 5,
                3 if ranks[1] == 2 => 4,
                3 => 3,
                2 if ranks[1] == 2 => 2,
                2 => 1,
                _ => 0,
            } << 29;
            let bet :u32 = line[6..]
                .split_whitespace()
                .next()
                .and_then(|num_str| num_str.parse().ok())
                .expect("Invalid number");
            (power, bet)
        })
        .collect::<Vec<_>>();
    hands.sort_unstable();

    let result = hands
        .into_iter()
        .enumerate()
        .map(|(i, (_power, bet))| bet * (i as u32 + 1))
        .sum::<u32>();

    println!("{}", result);
}
