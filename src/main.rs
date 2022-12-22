// This program will calculate the median and mode of a list of integers
use rand::Rng;
use std::collections::HashMap;

fn main() {
    // Create a list of integers that is unsorted
    let mut list: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..9 {
        list.push(rng.gen_range(0..10));
    }

    list.sort();

    println!("The list is {:?}", list);
    println!("The median is {}", median(&list));
    println!("The mode is {}", mode(&list))
}

fn median(list: &Vec<i32>) -> f32 {
    let list_length: i32 = list.len() as i32;
    // Check if even or odd
    if list_length % 2 != 0 {
        let middle_index: i32 = list_length / 2;
        let median_value: Option<&i32> = list.get(middle_index as usize);
        match median_value {
            Some(median_value) => return *median_value as f32,
            None => panic!("No median value found"),
        }
    } else {
        let upper_index: i32 = list_length / 2;
        let lower_index: i32 = upper_index - 1;
        let lower_value: Option<&i32> = list.get(lower_index as usize);
        let upper_value: Option<&i32> = list.get(upper_index as usize);
        match (lower_value, upper_value) {
            (Some(lower_value), Some(upper_value)) => {
                return (*lower_value as f32 + *upper_value as f32) / 2.0
            }
            _ => panic!("No median value found"),
        }
    }
}

fn mode(list: &Vec<i32>) -> i32 {
    let mut mode: i32 = 0;
    let mut mode_count: i32 = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();
    for num in list {
        let count = map.entry(*num).or_insert(0);
        *count += 1;
    }
    for (key, value) in &map {
        if value > &mode_count {
            mode = *key;
            mode_count = *value;
        }
    }
    mode
}
