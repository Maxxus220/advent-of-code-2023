use std::fs;

fn main() {
    let file_contents = fs::read_to_string("./day4/input").expect("File does not exist");
    let total_card_values: u32 = file_contents
        .lines()
        .map(parse_cards)
        .map(get_card_values)
        .sum();
    println!("The card values total to: {}", total_card_values);
}

/**
 * Takes a line, removes the Card #: prefix and splits the two sides (winning_numbers, numbers)
 * into two separate Vec<&str> that are then put in a Vec<Vec<&str>> where [0] is the winning numbers
 * and [1] are the numbers. (i.e. "Card 1: 3 5 10 2 | 1 6 13 20 39 50" [0][2]=10, [1][3]=20, [0][3]=2)
 */
fn parse_cards(line: &str) -> Vec<Vec<&str>> {
    line.get(line.find(':').unwrap() + 1..)
        .unwrap()
        .split('|')
        .map(|number_list| {
            number_list
                .trim()
                .split(' ')
                .filter_map(|number| {
                    let trimmed = number.trim();
                    get_non_empty_string(trimmed)
                })
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>()
}

/**
 * Takes a Vec<Vec<&str>> with two Vec<&str> within. Index 0 being the winning numbers and [1] being
 * the numbers. It then calculates the score for that card based on how many numbers in the numbers list.
 * match a number in the winning numbers list. If zero numbers match then 0 otherwise the score
 * is 2^(n-1) where n is the number of numbers that match a winning number.
 */
fn get_card_values(card: Vec<Vec<&str>>) -> u32 {
    let winning_nums = &card[0];
    let nums = &card[1];

    let mut num_matches = 0;
    for num in nums {
        if winning_nums.contains(num) {
            num_matches += 1;
        }
    }

    match num_matches {
        0 => 0,
        _ => 2_u32.pow(num_matches - 1),
    }
}

/**
 * Takes a &str and returns it as a Some if not empty otherwise None
 */
fn get_non_empty_string(string: &str) -> Option<&str> {
    if string.is_empty() {
        None
    } else {
        Some(string)
    }
}
