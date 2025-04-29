#![allow(dead_code)]

/*

Prefix Sums

Prefix sum is a technique that can be used on arrays (of numbers). The idea is to create an array prefix where prefix[i] is the sum of all elements up to the index i (inclusive). For example, given nums = [5, 2, 1, 6, 3, 8], we would have prefix = [5, 7, 8, 14, 17, 25].

Given an array nums,

prefix = [nums[0]]
for (int i = 1; i < nums.length; i++)
    prefix.append(nums[i] + prefix[prefix.length - 1])

*/

fn prefix_queries(nums: Vec<i32>, queries: Vec<[i32; 2]>, limit: i32) -> Vec<bool> {
    let mut answer = vec![];
    let mut prefix = vec![nums[0]];

    for (_, value) in nums.iter().skip(1).enumerate() {
        prefix.push(*value + prefix[prefix.len() - 1]);
    }

    for [x, y] in queries {
        let curr = prefix[y as usize] - prefix[x as usize] + nums[x as usize];
        answer.push(curr < limit);
    }

    return answer;
}

pub fn run_prefix_queries() {
    let nums = vec![1, 6, 3, 2, 7, 2];
    let queries = vec![[0, 3], [2, 5], [2, 4]];
    let limit = 13;
    let answer = vec![true, false, true];

    let my_answer = prefix_queries(nums, queries, limit);
    println!("My answer: {:?} actual: {:?}", my_answer, answer);
    assert_eq!(answer, my_answer);
}

/* 2270. Number of Ways to Split an Array
https://leetcode.com/problems/number-of-ways-to-split-array/
Given an integer array nums, find the number of ways to split the array into two parts so that the first section has a sum greater than or equal to the sum of the second section. The second section should have at least one number.
*/

fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    let mut prefix = vec![nums[0]];
    for (_, value) in nums.iter().skip(1).enumerate() {
        prefix.push(*value + prefix[prefix.len() - 1]);
    }

    let mut answer = 0;
    for index in 1..n {
        let left_section = prefix[index];
        let right_section = prefix[n - 1] - prefix[index];
        if left_section >= right_section {
            answer += 1;
        }
    }

    return answer;
}

pub fn run_ways_to_split_array() {
    let nums = vec![10, 4, -8, 7];
    let my_answer = ways_to_split_array(nums);
    let answer = 2;
    println!("my answer {} actual {}", my_answer, answer);
    assert_eq!(my_answer, answer);
}

/*
prefix queries using integer instead of an array
"Ways to split array"
*/

fn ways_to_split_array_int(nums: Vec<i32>) -> i32 {
    let mut answer = 0;
    let mut left_section = 0;
    let mut total = 0;

    for (_, value) in nums.iter().enumerate() {
        total += value;
    }

    for (_, value) in nums.iter().skip(nums.len()).enumerate() {
        left_section += *value;
        let right_section = total - left_section;
        if left_section >= right_section {
            answer += 1;
        }
    }

    return answer;
}

pub fn run_ways_to_split_array_int() {
    let nums = vec![1, 6, 3, 2, 7, 2];
    let queries = vec![[0, 3], [2, 5], [2, 4]];
    let limit = 13;
    let answer = vec![true, false, true];

    let my_answer = prefix_queries(nums, queries, limit);
    println!("My answer: {:?} actual: {:?}", my_answer, answer);
    assert_eq!(answer, my_answer);
}

/*
Running Sum of 1d Array

Given an array nums. We define a running sum of an array as runningSum[i] = sum(nums[0]…nums[i]).

Return the running sum of nums.

*/
fn get_prefix_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix_sum = vec![nums[0]];

    for (i, value) in nums.iter().enumerate().skip(1) {
        prefix_sum.push(*value + prefix_sum[i - 1]);
    }

    return prefix_sum;
}

pub fn run_get_prefix_sum() {
    let nums = vec![1, 2, 3, 4];
    let output = [1, 3, 6, 10];
    let my_ans = get_prefix_sum(nums);
    println!("my answer: {:?} actual {:?}", my_ans, output);
    assert_eq!(my_ans, output);

    let nums2 = vec![1, 1, 1, 1, 1];
    let output2 = [1, 2, 3, 4, 5];
    let my_ans2 = get_prefix_sum(nums2);
    println!("my answer: {:?} actual {:?}", my_ans2, output2);
    assert_eq!(my_ans2, output2);

    let nums3 = vec![3, 1, 2, 10, 1];
    let output3 = [3, 4, 6, 16, 17];
    let my_ans3 = get_prefix_sum(nums3);
    println!("my answer: {:?} actual {:?}", my_ans3, output3);
    assert_eq!(my_ans3, output3);
}

/*
Minimum Value to Get Positive Step by Step Sum

Given an array of integers nums, you start with an initial positive value startValue.

In each iteration, you calculate the step by step sum of startValue plus elements in nums (from left to right).

Return the minimum positive value of startValue such that the step by step sum is never less than 1.

*/

fn calculate_minimum_start_value(nums: Vec<i32>) -> i32 {
    // its just a prefix sum
    // along the way, check if any value is less than 1. If so, calculate the startValue such that 1 = startValue + lowest_num
    use std::cmp;

    let mut current = nums[0]; // current sum
    let mut start_value = 1;

    if current < 1 {
        start_value = 1 + nums[0].abs();
    }

    for (_, value) in nums.iter().enumerate().skip(1) {
        current += *value;

        if current < 1 {
            start_value = cmp::max(start_value, 1 + current.abs())
        }
    }

    return start_value;
}

pub fn run_calculate_minimum_start_value() {
    let nums = vec![-3, 2, -3, 4, 2];
    let answer = 5;
    let my_answer = calculate_minimum_start_value(nums);
    println!("my answer: {} answer {}", my_answer, answer);
    assert_eq!(answer, my_answer);

    let nums2 = vec![1, 2];
    let answer2 = 1;
    let my_answer2 = calculate_minimum_start_value(nums2);
    println!("my answer: {} answer {}", my_answer2, answer2);
    assert_eq!(answer2, my_answer2);

    let nums3 = vec![1, -2, -3];
    let answer3 = 5;
    let my_answer3 = calculate_minimum_start_value(nums3);
    println!("my answer: {} answer {}", my_answer3, answer3);
    assert_eq!(answer3, my_answer3);
}

/*
K Radius Subarray Averages

Yikes

You are given a 0-indexed array nums of n integers, and an integer k.

The k-radius average for a subarray of nums centered at some index i with the radius k is the average of all elements in nums between the indices i - k and i + k (inclusive). If there are less than k elements before or after the index i, then the k-radius average is -1.

Build and return an array avgs of length n where avgs[i] is the k-radius average for the subarray centered at index i.

The average of x elements is the sum of the x elements divided by x, using integer division. The integer division truncates toward zero, which means losing its fractional part.

    For example, the average of four elements 2, 3, 1, and 5 is (2 + 3 + 1 + 5) / 4 = 11 / 4 = 2.75, which truncates to 2.
*/

fn k_radius_sub_avg(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if k == 0 {
        return nums;
    }

    let mut output = vec![-1; nums.len()];
    let window_size = (2 * k) + 1;
    // the first k values are -1

    // create a prefix array of all sums
    let mut prefix_sums: Vec<i64> = vec![nums[0] as i64];
    for (i, value) in nums.iter().enumerate().skip(1) {
        // println!("size of prefix_sums = {}", prefix_sums.len());
        // println!("to push: {}", (*value as i64 + prefix_sums[i - 1]));
        prefix_sums.push(*value as i64 + prefix_sums[i - 1]);
        // (2 * k) represents the highest index within the radius of n
        // when this statement is true, we have enough data to start calculating averages
        // Begin populating the averages
        if i as i32 >= window_size - 1 {
            // 2 * k works here too, but semantically "window size minus 1" is a better fit
            // the calculated data is to be stored at the 'center' of the subarray, which is located at (i-k)
            // The value to store is equal to the sum of the current subarray, divided by the length of the subarray
            let mut average = prefix_sums[i];
            if i as i32 > (2 * k) {
                // if we need to subtract something
                let index = i as i32 - window_size; // index of the sum we need to subtract from the average sum
                average -= prefix_sums[index as usize];
            }
            average /= window_size as i64;

            output[i - k as usize] = average as i32;
        }
    }

    return output;
}

pub fn run_k_radius_sub_avg() {
    let nums = vec![7, 4, 3, 9, 1, 8, 5, 2, 6];
    let k = 3;
    let answer = vec![-1, -1, -1, 5, 4, 4, -1, -1, -1];
    let averages = k_radius_sub_avg(nums, k);
    println!("my answer: {:?} \nanswer: {:?}", averages, answer);

    // let nums2 = vec![100000; 100000];
    // let k2 = 40000;
    // let averages2 = k_radius_sub_avg(nums2, k2);
}
