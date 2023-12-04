use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main()-> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);
    let mut total_points = 0;

    for line in reader.lines(){
        let line = line?;
        let splitString: Vec<&str> = line.split(":").collect();
        let mut point_counter = 0;
        let mut winning_point = 1;

        let ticketPart: Vec<&str> = splitString[1].split("|").collect();
        let winning_numbers: Vec<&str> = ticketPart[0].split(" ").collect();
        let player_numbers: Vec<&str> = ticketPart[1].split(" ").collect();

        let winning_numbers: Vec<i32> = winning_numbers
        .iter()
        .filter_map(|n| n.parse().ok())
        .collect();

        let player_numbers: Vec<i32> = player_numbers
        .iter()
        .filter_map(|n| n.parse().ok())
        .collect();

  

    for &winning_number in &winning_numbers {
        println!("test: {}", winning_number);
        if player_numbers.contains(&winning_number) {
            point_counter = winning_point;

            winning_point *= 2;

        }
    }

    total_points += point_counter;

  
        
    }

    println!("Partie 1, Points totaux: {}",total_points );
    Ok(())
}
