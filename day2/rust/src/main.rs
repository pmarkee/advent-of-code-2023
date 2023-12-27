struct GameValidity {
    game_id: u32,
    valid: bool,
}

fn main() {
    let lines: Vec<_> = std::fs::read_to_string("../input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let sum: u32 = lines
        .iter()
        .map(input_line_to_game_validity)
        .filter_map(|validity| match validity.valid {
            true => Some(validity.game_id),
            false => None,
        })
        .sum();
    println!("Solution: {sum}");
}

fn input_line_to_game_validity(line: &String) -> GameValidity {
    let game_id_and_rounds: Vec<_> = line.split(":").collect();
    let game_id: u32 = game_id_and_rounds[0].split(" ").collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();
    let raw_rounds = game_id_and_rounds[1].trim();

    let valid = raw_rounds
        .split("; ")
        .flat_map(|round| round.split(", "))
        .all(|reveal| {
            let count_and_color: Vec<_> = reveal.split(" ").collect();
            let count: u32 = count_and_color[0].parse().unwrap();
            let color = count_and_color[1];
            match color {
                "red" => count <= 12,
                "green" => count <= 13,
                "blue" => count <= 14,
                _ => unreachable!("Only red, green, blue should be possible"),
            }
        });
    GameValidity { game_id, valid }
}
