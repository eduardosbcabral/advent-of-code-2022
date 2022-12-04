use std::{fs, collections::HashSet};

fn main() {
    let file_path = "src/input";

    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let total_priority_1 = input
        .lines()
        .map(|x| x.split_at(x.len()/2))
        .map(|(x, y)| (x.chars().collect::<HashSet<_>>(), y.chars().collect::<HashSet<_>>()))
        .map(|(x, y)| *(&x & &y).iter().next().unwrap())
        .map(get_priority)
        .sum::<u32>();

    println!("{:?}", total_priority_1);

    let mut total_priority_2 = 0;
    for i in input.lines().collect::<Vec<_>>().chunks(3) {
        let mut all = HashSet::new();
        i.iter().for_each(|x| all.extend(x.chars()));
        i.iter().for_each(|x| all.retain(|y| x.contains(*y)));
        
        total_priority_2 += get_priority(*all.iter().next().unwrap());
    }

    println!("{:?}", total_priority_2);
}

fn get_priority(char: char) -> u32 {
    match char.is_uppercase() {
        true => char as u32 - 38,
        false => char as u32 - 96
    }
}