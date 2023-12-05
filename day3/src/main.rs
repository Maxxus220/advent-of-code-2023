use std::cmp;
use std::fs;

#[derive(Debug)]
struct NumberPosition {
    start_x: u32,
    end_x: u32,
    y: u32,
}

#[derive(Debug)]
struct SymbolPosition {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct NumberInfo {
    value: u32,
    point: NumberPosition,
}

fn main() {
    let file_contents = fs::read_to_string("./input").expect("File does not exist");

    let mut numbers = Vec::<NumberInfo>::new();
    let mut symbols = Vec::<SymbolPosition>::new();

    let mut in_num = false;
    let mut current_number: u32 = 0;
    let mut start_index: u32 = 0;
    let mut end_index: u32 = 0;

    let mut line_index = 0;
    let mut letter_index = 0;
    for letter in file_contents.chars() {
        if letter.is_numeric() {
            current_number *= 10;
            current_number += letter.to_digit(10).unwrap();

            if !in_num {
                start_index = letter_index as u32;
            }

            in_num = true;
        } else {
            if in_num {
                end_index = std::cmp::max(0, letter_index - 1) as u32;

                let number = NumberInfo {
                    value: current_number,
                    point: NumberPosition {
                        start_x: start_index,
                        end_x: end_index,
                        y: line_index as u32,
                    },
                };

                numbers.push(number);

                current_number = 0;
                start_index = 0;
                end_index = 0;

                in_num = false;
            }

            if letter == '\n' {
                line_index += 1;
                letter_index = -1;
            } else if letter != '.' && letter != '\r' {
                let symbol = SymbolPosition {
                    x: letter_index as u32,
                    y: line_index as u32,
                };
                symbols.push(symbol);
            }
        }

        letter_index += 1;
    }

    // for number in &numbers {
    //     println!("{:?}", number);
    // }
    // for symbol in &symbols {
    //     println!("{:?}", symbol);
    // }

    let valid_numbers: Vec<&NumberInfo> = numbers
        .iter()
        .filter(|number| {
            let min_x = if number.point.start_x == 0 {
                0
            } else {
                number.point.start_x - 1
            };
            let max_x = number.point.end_x + 1;
            let min_y = if number.point.y == 0 {
                0
            } else {
                number.point.y - 1
            };
            let max_y = number.point.y + 1;
            for symbol in &symbols {
                if symbol.x >= min_x && symbol.x <= max_x && symbol.y >= min_y && symbol.y <= max_y
                {
                    return true;
                }
            }
            false
        })
        .collect();

    for number in &valid_numbers {
        println!("{:?}", number);
    }

    let sum = valid_numbers
        .iter()
        .fold(0, |sum, number| sum + number.value);

    println!("The solution is: {}", sum);
}
