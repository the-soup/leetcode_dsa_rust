/*
Quick and dirty implementation of a simple cipher

*/

use std::collections::{HashMap, HashSet};

fn simple_string_hash(input: String) -> Vec<u8> {
    let lower_input = input.to_ascii_lowercase();
    let my_string: Vec<u8> = lower_input
        .bytes()
        .map(|x| if x == 32 { 0 } else { x - 96 })
        .collect();

    let mut output: Vec<u8> = vec![];

    for (index, value) in my_string.iter().enumerate() {
        output.push(value + index as u8 + 1);
    }

    println!("{:?}", output);

    return output;
}

pub fn run_simple_string_hash() {
    let my_str = "My secret message".to_string();
    simple_string_hash(my_str);
}

/*
Two Sum, Hash map style
*/

fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (i, value) in nums.iter().enumerate() {
        let complement = target - value;
        if map.contains_key(&complement) {
            if let Some(index) = map.get(&complement) {
                return vec![*index as i32, i as i32];
            };
        }
        map.insert(*value, i);
    }
    return vec![-1, -1];
}

pub fn run_two_sum_hash() {
    let nums = vec![5, 2, 7, 10, 3, 9];
    let target = 8;
    let answer = vec![0, 4];
    let result = two_sum_hash(nums, target);
    println!("my answer {:?} answer {:?}", result, answer);

    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let answer = vec![0, 1];
    let result = two_sum_hash(nums, target);
    println!("my answer {:?} answer {:?}", result, answer);
}

/*
First Letter to Appear Twice
*/

fn duplicate_letters(s: String) -> char {
    let mut set = HashSet::<char>::new();
    for letter in s.chars().into_iter() {
        if set.contains(&letter) {
            return letter;
        } else {
            set.insert(letter);
        }
    }

    return 'ë'; // s is guaranteed to contain a duplicate, so return e with umlaut - this means error
}

pub fn run_duplicate_letters() {
    let letters = "abccbaacz".to_string();
    let letter = 'c';
    let duplicate_letter = duplicate_letters(letters);
    println!("my ans: {} ans: {}", duplicate_letter, letter);
    assert_eq!(letter, duplicate_letter);
}

fn check_if_pangram(sentence: String) -> bool {
    use std::collections::HashSet;

    let mut set = HashSet::new();

    for c in sentence.chars().into_iter() {
        if !set.contains(&c) {
            set.insert(c);
        }
    }

    return set.len() == 26;
}

pub fn run_check_if_pangram() {
    let sentence = "thequickbrownfoxjumpsoverthelazydog".to_string();
    let answer = true;
    let my_answer = check_if_pangram(sentence);
    println!("my ans: {} answer {}", my_answer, answer);
    assert_eq!(answer, my_answer);

    let sentence = "leetcode".to_string();
    let answer = false;
    let my_answer = check_if_pangram(sentence);
    println!("my ans: {} answer {}", my_answer, answer);
    assert_eq!(answer, my_answer);
}

/*
Missing number

Given an array nums containing n distinct numbers in the range [0, n], return the only number in the range that is missing from the array.

*/
fn missing_number(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let nums_len = nums.len();

    if nums_len == 1 {
        if nums[0] == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    let mut set: HashSet<usize> = HashSet::default();
    for num in nums {
        set.insert(num as usize);
    }

    for i in 0..=nums_len {
        if !set.contains(&i) {
            return i as i32;
        }
    }

    return 1;
}

pub fn run_missing_number() {
    let nums = vec![3, 0, 1];
    let answer = 2;
    let my_answer = missing_number(nums);
    println!("my answer {} answer {}", my_answer, answer);

    let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
    let answer = 8;
    let my_answer = missing_number(nums);
    println!("my answer {} answer {}", my_answer, answer);

    let nums = vec![0];
    let answer = 1;
    let my_answer = missing_number(nums);
    println!("my answer {} answer {}", my_answer, answer);
}
