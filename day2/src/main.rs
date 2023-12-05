use std::fs;

fn main() {

    let file_contents = fs::read_to_string("./input").expect("File does not exist");
    let file_lines = file_contents.split('\n').filter(|x| x.contains("Game"));

    let mut sum = 0;
    for game in file_lines {
        let mut maxes_of_colors = vec![("red", 0), ("green", 0), ("blue", 0)];

        let start_index = game.find(": ").unwrap() + 2;
        let line = &game[start_index..];
        let sets = line.split(';');
        for set in sets {
            let color_counts = set.split(',');
            for color_count in color_counts {
                let color_count: Vec<&str> = color_count.trim().split(' ').collect();
                println!("Color count: {:?}", color_count);
                for max_of_color in maxes_of_colors.iter_mut() {
                    if max_of_color.0 == color_count[1] && max_of_color.1 < color_count[0].parse::<u32>().expect("Not a number") {
                        max_of_color.1 = color_count[0].parse::<u32>().expect("Not a number")
                    }
                }
            }
        }
        let mut product = 1;
        for max in maxes_of_colors {
            product *= max.1;
        }
        sum += product;
    }

    println!("Sum of game products: {}", sum);
}
