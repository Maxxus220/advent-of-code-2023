use std::fs;

fn main() {
    let maxes_of_colors = vec![("red", 12), ("green", 13), ("blue", 14)];

    let file_contents = fs::read_to_string("./input").expect("File does not exist");
    let file_lines = file_contents.split("\n").into_iter().filter(|x| x.contains("Game"));

    let mut valid_games = Vec::<u32>::new();
    for (index, game) in file_lines.enumerate() {
        let mut valid = true;

        let start_index = game.find(": ").unwrap() + 2;
        let line = &game[start_index..];
        let sets = line.split(";");
        for set in sets {
            let color_counts = set.split(",");
            for color_count in color_counts {
                let color_count: Vec<&str> = color_count.trim().split(" ").collect();
                println!("Color count: {:?}", color_count);
                for max_of_color in &maxes_of_colors {
                    if max_of_color.0 == color_count[1] && max_of_color.1 < color_count[0].parse::<u32>().expect("Not a number") {
                        valid = false;
                    }
                }
            }
        }
        if valid {
            valid_games.push((index + 1) as u32);
        }
    }

    println!("Valid games: {:?}", valid_games);
    
    let mut sum = 0;
    for game_indice in valid_games {
        sum += game_indice;
    }
    println!("Sum of game indices: {}", sum);
}
