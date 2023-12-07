use std::fs;

struct CardSolution {
    card_id: u32,
    cards_won: Vec<u32>,
}

struct CardNumbers<'a> {
    winning_numbers: Vec<&'a str>,
    playing_numbers: Vec<&'a str>,
}

struct Card<'a> {
    card_id: u32,
    card_numbers: CardNumbers<'a>,
}

fn main() {
    let file_contents = fs::read_to_string("./day4/input").expect("File does not exist");
    let mut cards: Vec<Card> = file_contents.lines().map(parse_cards).collect();

    let mut previous_card_solutions: Vec<CardSolution> = Vec::new();

    let mut num_cards: u32 = 0;

    while let Some(card) = cards.pop() {
        num_cards += 1;

        let cached_solution = previous_card_solutions
            .iter()
            .find(|element| element.card_id == card.card_id);
        match cached_solution {
            Some(solution) => {
                push_card_solution(solution, &mut cards);
            }
            None => {
                let solution = solve_card(card);
                push_card_solution(&solution, &mut cards);
                previous_card_solutions.push(solution);
            }
        }
    }

    println!("You got {} cards!", num_cards);
}

/**
 * Takes a line and parses out the card #, winning numbers, and playing numbers.
 */
fn parse_cards(line: &str) -> Card {
    let card_id = line
        .get(4..line.find(':').unwrap())
        .unwrap()
        .trim()
        .parse::<u32>()
        .expect("Not a number");

    let card_numbers = line
        .get(line.find(':').unwrap() + 1..)
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
        .collect::<Vec<Vec<&str>>>();

    Card {
        card_id,
        card_numbers: CardNumbers {
            winning_numbers: card_numbers[0].clone(),
            playing_numbers: card_numbers[1].clone(),
        },
    }
}

/**
 * Takes a card and figures out what cards are won by it.
 */
fn solve_card(card: Card) -> CardSolution {
    let mut num_matches: u32 = 0;
    for num in &card.card_numbers.playing_numbers {
        if card.card_numbers.winning_numbers.contains(num) {
            num_matches += 1;
        }
    }

    let mut cards_won: Vec<u32> = Vec::new();
    for card_id in card.card_id + 1..card.card_id + num_matches + 1 {
        cards_won.push(card_id);
    }

    CardSolution {
        card_id: card.card_id,
        cards_won,
    }
}

/**
 * Takes a &str and returns it as a Some if not empty otherwise None.
 */
fn get_non_empty_string(string: &str) -> Option<&str> {
    if string.is_empty() {
        None
    } else {
        Some(string)
    }
}

/**
 * Pushes all won cards from a CardSolution to a Vec<Card>.
 */
fn push_card_solution(solution: &CardSolution, cards: &mut Vec<Card>) {
    solution.cards_won.iter().for_each(|won_card_id| {
        cards.push(Card {
            card_id: *won_card_id,
            card_numbers: CardNumbers {
                winning_numbers: Vec::new(),
                playing_numbers: Vec::new(),
            },
        })
    });
}
