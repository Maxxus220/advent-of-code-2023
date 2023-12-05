use std::fs;

fn main() {
    let valid_digit_words = ["zero","one","two","three","four","five","six","seven","eight","nine"];

    let file_contents = fs::read_to_string("./input").expect("File does not exist");
    let file_lines = file_contents.lines();

    let mut sum = 0;
    for line in file_lines {
        let mut digits = Vec::<(usize, char)>::new();

        for (index, char) in line.chars().enumerate() {
            if char.is_numeric() {
                digits.push((index, char));
            }
        }

        for (valid_digit_index, valid_digit_word) in valid_digit_words.iter().enumerate() {
            let digit_char = std::char::from_digit(valid_digit_index as u32, 10).unwrap();
            let digit_occurences = line.match_indices(valid_digit_word);
            for (digit_occurrence_index, _) in digit_occurences {
                digits.push((digit_occurrence_index, digit_char));
            }
        }

        digits.sort_by(|a, b| a.0.cmp(&b.0));

        // Skip if no numbers on line
        if digits.is_empty() {
            continue;
        }

        let first_number = digits[0].1;
        let last_number = digits[digits.len() - 1].1;
        let number = first_number.to_string() + last_number.to_string().as_str();
        sum += number.parse::<u32>().expect("Not a number");
    }

    println!("The sum is: {}", sum);

}
