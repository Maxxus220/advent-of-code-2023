use std::cmp;
use std::error::Error;
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
struct SymbolInfo {
    value: char,
    position: SymbolPosition,
}

#[derive(Debug)]
struct NumberInfo {
    value: u32,
    position: NumberPosition,
}

fn main() {
    let file_contents = fs::read_to_string("./day3/input").expect("File does not exist");

    let (numbers, symbols) = parse_part_numbers_and_symbols(file_contents);

    let sum: u32 = symbols
        .iter()
        .map(|symbol| get_gear_ratio(symbol, &numbers))
        .filter_map(|gear_result| gear_result.ok())
        .sum();

    println!("The sum is {}", sum);
}

fn parse_part_numbers_and_symbols(data: String) -> (Vec<NumberInfo>, Vec<SymbolInfo>) {
    let mut numbers = Vec::<NumberInfo>::new();
    let mut symbols = Vec::<SymbolInfo>::new();

    let mut in_num = false;
    let mut current_number: u32 = 0;
    let mut start_index: u32 = 0;
    let mut end_index: u32 = 0;

    let mut line_index: i32 = 0;
    let mut letter_index: i32 = 0;
    for letter in data.chars() {
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
                    position: NumberPosition {
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
                let symbol = SymbolInfo {
                    value: letter,
                    position: SymbolPosition {
                        x: letter_index as u32,
                        y: line_index as u32,
                    },
                };
                symbols.push(symbol);
            }
        }

        letter_index += 1;
    }

    (numbers, symbols)
}

fn get_gear_ratio(symbol: &SymbolInfo, numbers: &Vec<NumberInfo>) -> Result<u32, &'static str> {
    if symbol.value != '*' {
        return Err("Not a gear");
    }

    let mut part_numbers = Vec::<u32>::new();
    for number in numbers {
        if number_is_part_of_gear(symbol, number) {
            if part_numbers.len() >= 2 {
                return Err("Not a gear");
            }

            part_numbers.push(number.value);
        }
    }

    if part_numbers.len() != 2 {
        return Err("Not a gear");
    }

    Ok(part_numbers[0] * part_numbers[1])
}

fn number_is_part_of_gear(symbol: &SymbolInfo, number: &NumberInfo) -> bool {
    let max_x = number.position.end_x + 1;
    let min_x = if number.position.start_x == 0 {
        0
    } else {
        number.position.start_x - 1
    };
    let max_y = number.position.y + 1;
    let min_y = if number.position.y == 0 {
        0
    } else {
        number.position.y - 1
    };
    symbol.position.x <= max_x
        && symbol.position.x >= min_x
        && symbol.position.y <= max_y
        && symbol.position.y >= min_y
}
