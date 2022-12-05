use std::{fs, collections::HashSet};

fn main() {
    let file_path = "src/input";

    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total_sum_1: u32 = 0;
    let mut total_sum_2: u32 = 0;

    for line in input.lines() {
        let (first_half, second_half) = line.rsplit_once(',').unwrap();
        let (first_section_start_string, first_section_end_string) = 
            first_half.rsplit_once('-').unwrap();
        let (second_section_start_string, second_section_end_string) =
            second_half.rsplit_once('-').unwrap();
            
        let first_section_start: u32 = first_section_start_string.parse().unwrap();
        let first_section_end: u32 = first_section_end_string.parse().unwrap();
        let second_section_start: u32 = second_section_start_string.parse().unwrap();
        let second_section_end: u32 = second_section_end_string.parse().unwrap();

        let mut a: HashSet<u32> = HashSet::new();
        for v in first_section_start..first_section_end + 1 {
            a.insert(v);
        }

        let mut b: HashSet<u32> = HashSet::new();
        for v in second_section_start..second_section_end + 1 {
            b.insert(v);
        }

        if a.is_subset(&b) || a.is_superset(&b) {
            total_sum_1 += 1;
        }

        if !a.is_disjoint(&b) {
            total_sum_2 += 1;
        }
    }
        
    println!("{:?}", total_sum_1);
    println!("{:?}", total_sum_2);
}
