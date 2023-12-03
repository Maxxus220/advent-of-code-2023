use std::fs;

enum Colors {
    Red,
    Green,
    Blue
}

fn main() {
    let maxes_of_colors = vec![(Colors::Red, 12), (Colors::Green, 13), (Colors::Blue, 14)];

    let file_contents = fs::read_to_string("./input").expect("File does not exist");
    let file_lines = file_contents.split("\n").into_iter().filter(|x| x.contains("Game"));

    let valid_games = Vec::<u32>::new();
    for (index, line) in file_lines.enumerate() {
        let start_index = line.find(": ").unwrap() + 2;
        let line = line[start_index..];
        let sets = line.split(";");
        for set in sets {
            set.split(",");
        }
    }
}
