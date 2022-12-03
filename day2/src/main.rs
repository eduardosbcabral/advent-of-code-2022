use std::fs;

fn main() {
    let file_path = "src/input";

    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let total_score_1 = input
        .replace('\r', "")
        .split('\n')
        .map(get_round_score_1)
        .sum::<u32>();

    // Anwser 1: 11873
    println!("Score 1: {}", total_score_1);


    let total_score_2 = input
        .replace('\r', "")
        .split('\n')
        .map(get_round_score_2)
        .sum::<u32>();

    // Anwser 2: 12014
    println!("Score 2: {}", total_score_2);
}

fn get_round_score_1(round: &str) -> u32 {
    let mut chars = round.chars(); // A Y
    let enemy = chars.next().unwrap();
    let me = chars.next_back().unwrap();

    if wins(me) == enemy {
        get_choice_score(me) + 6
    } else if wins(enemy) == me {
        get_choice_score(me)
    } else {
        get_choice_score(me) + 3   
    }
}

fn get_round_score_2(round: &str) -> u32 {
    let mut chars = round.chars(); // A Y
    let enemy = chars.next().unwrap();
    let me = chars.next_back().unwrap();

    match me {
        'Z' => get_choice_score(loses(enemy)) + 6,
        'X' => get_choice_score(wins(enemy)),
        _ => get_choice_score(enemy) + 3
    }
}

fn wins(char: char) -> char {
    match char {
        'A' => 'Z',
        'B' => 'X',
        'C' => 'Y',
        'X' => 'C',
        'Y' => 'A',
        'Z' => 'B',
        _ => ' '
    }
}

fn loses(char: char) -> char {
    match char {
        'A' => 'Y',
        'B' => 'Z',
        'C' => 'X',
        _ => ' '
    }
}

// A, X = rock (1 point)
// B, Y = paper (2 points)
// C, Z = scissor (3 points)
fn get_choice_score(char: char) -> u32 {
    match char {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => 0
    }
}