/*
Quick and dirty implementation of a simple cipher

*/

use crate::common;

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
    use std::collections::HashMap;

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
    use std::collections::HashSet;

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

/*
Counting elements

Given an integer array arr, count how many elements x there are, such that x + 1 is also in arr. If there are duplicates in arr, count them separately.

*/
fn counting_elements(arr: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    let mut count = 0;

    for n in &arr {
        set.insert(*n);
    }

    for n in arr {
        let x = n + 1;
        if set.contains(&x) {
            count += 1;
        }
    }

    return count;
}

pub fn run_counting_elements() {
    common::evaluate_vec_i32(vec![1,2,3], 2, counting_elements);

}

/*
Intersection of Multiple Arrays

Given a 2D integer array nums where nums[i] is a non-empty array of distinct positive integers, return the list of integers that are present in each array of nums sorted in ascending order. 
*/

fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashMap;

    let mut output = vec![];
    let mut count: HashMap<i32, i32> = HashMap::new();
    let nums_len = nums.len();

    // Count frequency of all values across the outer array
    for n in nums {
        for m in n {
            match count.get(&m) {
                Some(value) => {
                    count.insert(m, value + 1);
                },
                None => {
                    count.insert(m, 1);
                }
            }
        }
    }

    for (key, value) in count {
        if value == nums_len as i32 {
            output.push(key);
        }
    }

    output.sort();
    return output;
}

pub fn run_intersection() {
    let nums = vec![vec![3,1,2,4,5],vec![1,2,3,4],vec![3,4,5,6]];
    common::evaluate_nested_vec_i32(nums, vec![3,4], intersection);
}

/* 
1941. Check if All Characters Have Equal Number of Occurrences

Given a string s, return true if s is a good string, or false otherwise.

A string s is good if all the characters that appear in s have the same number of occurrences (i.e., the same frequency).

*/
fn occurrances_equal(s: String) -> bool {
    use std::collections::{HashMap, HashSet};

    let mut counts: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        // lets break this down
        // get a mutable value from the counts map, which returns on option. unwrap the option.
        // This gives us a reference to the value, and then we can dereference the value which
        // is now also mutable, and we can add 1 to the value.
        if counts.contains_key(&c) {
            *counts.get_mut(&c).unwrap() += 1;
        } else {
            counts.insert(c, 1);
        }

    }

    let mut frequencies: HashSet<i32> = HashSet::new();
    for n in counts.values() {
        frequencies.insert(*n);
    }

    println!("{:?}", frequencies);
    return frequencies.len() == 1;

}

pub fn run_occurrances_equal() {
    let s = "abacbc".to_string();
    let output = true;
    let my_answer = occurrances_equal(s);
    println!("my answer: {} answer {}", my_answer, output);
    assert_eq!(output, my_answer);
}

/* 
560. Subarray Sum Equals K

Given an array of integers nums and an integer k, return the total number of subarrays whose sum equals to k.

A subarray is a contiguous non-empty sequence of elements within an array.
*/
fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;

    if nums.len() == 1 {
        if nums[0] == k {
            return 1;
        } else {
            return 0;
        }
    }

    let mut counts = HashMap::with_capacity(nums.len());
    counts.insert(0, 1);
    let mut curr = 0;
    let mut output = 0;

    for n in nums {
        curr += n;
        if counts.contains_key(&{curr - k}) {
            output += counts.get(&{curr - k}).unwrap();
        }
        counts.insert(curr, counts.get(&curr).unwrap_or(&0) + 1);
    }

    return output;
}

pub fn run_subarray_sum() {
    let nums = vec![1, 2, 1, 2, 1];
    let k = 3;
    let answer = 4;
    let my_answer = subarray_sum(nums, k);
    println!("my_answer {} answer {}", my_answer, answer);
    assert_eq!(answer, my_answer);
}

/*
1248. Count Number of Nice Subarrays

Given an array of integers nums and an integer k. A continuous subarray is called nice if there are k odd numbers on it.

Return the number of nice sub-arrays.
TODO: review me!

*/
fn nice_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;
    let mut curr = 0;
    let mut output = 0;

    let mut counts = HashMap::new();
    counts.insert(0, 1);

    for n in nums {
        // keep track of the number of odd numbers we see.
        // If the value is even, the value is not incremented.
        curr += n % 2;
        if counts.contains_key(&{curr - k}) {
            output += counts.get(&{curr - k}).unwrap();
        }
        // in the map, map the current number of odd numbers seen in the array as the key
        // and increment the count.
        // so if we've seen 2 odd numbers
        counts.insert(curr, counts.get(&curr).unwrap_or(&0) + 1);
    }

    return output;
}

pub fn run_nice_subarrays() {
    let nums = vec![1,1,2,1,1];
    let k = 3;
    let answer = 2;
    let my_answer = nice_subarrays(nums, k);
    println!("my_answer: {} answer: {}", my_answer, answer);
}

/*
Find Players With Zero or One Losses

You are given an integer array matches where matches[i] = [winneri, loseri] indicates that the player winneri defeated player loseri in a match.

Return a list answer of size 2 where:

    answer[0] is a list of all players that have not lost any matches.
    answer[1] is a list of all players that have lost exactly one match.

The values in the two lists should be returned in increasing order.

Note:

    You should only consider the players that have played at least one match.
    The testcases will be generated such that no two matches will have the same outcome.


    1 <= matches.length <= 105
    matches[i].length == 2
    1 <= winneri, loseri <= 105
    winneri != loseri
    All matches[i] are unique.

    Time complexity O(n Log(n))
    Space complexity O(n)
*/

fn zero_or_one_losses(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::HashMap;

    if matches.len() == 1 {
        return vec![vec![matches[0][0]], vec![matches[0][1]]];
    }

    // losses will track the number of losses for each 'player'
    let mut losses = HashMap::new();

    for single in matches {
        let winner = single[0];
        let loser = single[1];

        // one single match contains [x,y] where X is the winner, and Y is the loser.
        // first, check the winner
        if !losses.contains_key(&winner) {
            losses.insert(winner, 0);
        }
        if !losses.contains_key(&loser) {
            losses.insert(loser, 1);
        } else {
            losses.insert(loser, losses.get(&loser).unwrap() + 1);
        }
    }

    let mut zero_loss = Vec::<i32>::new();
    let mut one_loss = Vec::<i32>::new();

    for key in losses.keys() {
        if losses.get(key).unwrap() == &0 {
            zero_loss.push(*key);
        }
        if losses.get(key).unwrap() == &1 {
            one_loss.push(*key);
        }
    }
    zero_loss.sort();
    one_loss.sort();

    return vec![zero_loss, one_loss];
}

pub fn run_zero_or_one_losses() {
    let matches = vec![vec![1,3],vec![2,3],vec![3,6],vec![5,6],vec![5,7],vec![4,5],vec![4,8],vec![4,9],vec![10,4],vec![10,9]];
    let answer = vec![vec![1,2,10],vec![4,5,7,8]];
    let my_answer = zero_or_one_losses(matches);
    println!("my answer {:?} answer {:?}", my_answer, answer);
}

/*
Find Players With Zero or One Losses

Same as above, but this time a solution using an array;
This has time complexity of O(n + k)

Counting Sort
https://en.wikipedia.org/wiki/Counting_sort

*/
fn count_losses_array(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let k = 100000 + 1;
    let mut output = vec![vec![]; 2];

    // Create an array with size matches.len(), and init each value to -1
    // We are counting losses, and the matches will have at most k losers
    let mut losses_count = vec![-1; k];

    for game in matches {
        let winner = game[0] as usize;
        let loser = game[1] as usize;

        // Update losing counts
        match losses_count[loser] {
            -1 => losses_count[loser] = 1,
            _ => losses_count[loser] += 1,
        }

        // Need to track if a winner has played a match
        // We do this by updating their default value to 0.
        if losses_count[winner] == -1 {
            losses_count[winner] = 0;
        }
    }

    for (player, losses) in losses_count.iter().enumerate() {
        match losses {
            0 => output[0].push(player as i32), // players that have played but never lost
            1 => output[1].push(player as i32), // players that have lost only once
            _ => {}
        }
    }

    return output;
}

pub fn run_count_losses_array() {
    let matches = vec![vec![1,3],vec![2,3],vec![3,6],vec![5,6],vec![5,7],vec![4,5],vec![4,8],vec![4,9],vec![10,4],vec![10,9]];
    let answer = vec![vec![1,2,10],vec![4,5,7,8]];
    let my_answer = count_losses_array(matches);
    println!("my answer {:?} answer {:?}", my_answer, answer);
}

/*

Largest Unique Number

Given an integer array nums, return the largest integer that only occurs once. If no integer occurs once, return -1.


1 <= nums.length <= 2000
0 <= nums[i] <= 1000

*/

fn largest_unique_number(nums: Vec<i32>) -> i32 {
    let mut output = -1;
    let mut count = vec![0; 1001]; // constraints state numbers can be as high as 1000. So we need at least 1000 slots in our array

    for num in nums {
        count[num as usize] += 1;
    }

    for (index, num) in count.iter().enumerate() {
        if *num == 1 {
            output = index as i32;
        }
    }

    return output;
}

pub fn run_largest_unique_number() {
    let nums = vec![5,7,3,9,4,9,8,3,1];
    let answer = 8;
    let my_answer = largest_unique_number(nums);
    println!("my_answer {}, answer {}", my_answer, answer);
    assert_eq!(my_answer, answer);

    let nums = vec![9,9,8,8];
    let answer = -1;
    let my_answer = largest_unique_number(nums);
    println!("my_answer {}, answer {}", my_answer, answer);
    assert_eq!(my_answer, answer);
}

/*

Maximum number of balloons

Given a string text, you want to use the characters of text to form as many instances of the word "balloon" as possible.

You can use each character in text at most once. Return the maximum number of instances that can be formed.


1 <= text.length <= 104
text consists of lower case English letters only.

*/
fn maximum_number_of_balloons(text: String) -> i32 {
    use std::collections::HashMap;
    use std::cmp::min;

    // To find the maximum occurrences of 'balloon' we just need
    // to count the frequency of each letter.
    let mut output = 100000;
    let balloon = vec!['b', 'a', 'l', 'o', 'n'];

    // ratios: B:1, A:1, L:2, O:2, N:1
    let mut map = HashMap::<char, i32>::new();
    for letter in balloon.clone() {
        map.insert(letter, 0);
    }

    for letter in text.chars() {
        if balloon.contains(&letter) {
            match map.get(&letter) {
                Some(value) => { map.insert(letter, value + 1);}
                None => { map.insert(letter, 1); }
            }
        }
    }

    for letter in balloon {
        // Check each value in the map
        if let Some(value) = map.get(&letter) {
            let mut new = *value; // temporary store for next computation
            if letter == 'l' || letter == 'o' { // There are two L and O in balloon, so we need to halve each occurance count
                new /= 2;
            }
            output = min(output, new);
        }
    }

    return output;
}

pub fn run_maximum_number_of_balloons() {
    let text = "nlaebolko".to_string();
    let answer = 1;
    let my_answer = maximum_number_of_balloons(text);
    println!("my answer {} answer {}", my_answer, answer);
    assert_eq!(my_answer, answer);

    let text = "loonbalxballpoon".to_string();
    let answer = 2;
    let my_answer = maximum_number_of_balloons(text);
    println!("my answer {} answer {}", my_answer, answer);
    assert_eq!(my_answer, answer);

    let text = "leetcode".to_string();
    let answer = 0;
    let my_answer = maximum_number_of_balloons(text);
    println!("my answer {} answer {}", my_answer, answer);
    assert_eq!(my_answer, answer);

    let text = "balon".to_string();
    let answer = 0;
    let my_answer = maximum_number_of_balloons(text);
    println!("my answer {} answer {}", my_answer, answer);
    assert_eq!(my_answer, answer);
}

/*

Contiguous Array

Given a binary array nums, return the maximum length of a contiguous subarray with an equal number of 0 and 1.

*/

fn contiguous_array(nums: Vec<i32>) -> i32 {
    let output = 0;
    let k = 0; // k will represent the mirror

    return output;
}

pub fn run_contiguous_array() {

}