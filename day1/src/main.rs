use std::fs;

fn main() {
    let file_path = "src/input";

    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let calculated_calories = get_calculated_calories(&input);

    let highest_one = calculated_calories.last().unwrap();

    // Reversing the array and taking the first 3
    //let highest_top_three: u32 = calculated_calories.iter().rev().take(3).sum();

    // Taking the last three using the spread operator
    let highest_top_three: u32 = calculated_calories[calculated_calories.len()-3..].iter().sum(); 
    println!("Highest calories carried by an elf: {}", highest_one);
    println!("Highest calories carried by the top three elfs: {}", highest_top_three);
}

fn get_calculated_calories(data: &str) -> Vec<u32> {
    let mut vec = data
        .replace('\r', "") // Replace the carriage return character
        .split("\n\n") // Split the elfs by empty lines
        // For each value we have a break line char (ex: 23\n10\n35\n5), so the function 'lines' iterate over by slicing using this exacly char.
        // For each value we parse to u32, unwrap it to get the contained value and sum all the values from each elf 
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum::<u32>()) 
        // Transforms to a collection
        .collect::<Vec<_>>();
    // Sort to get the highest values to the end of the collection
    vec.sort();
    vec
}