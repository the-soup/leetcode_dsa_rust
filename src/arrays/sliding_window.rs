#![allow(dead_code)]

/* -- Sum of Window -- */
fn sum_of_window(nums: &Vec<u32>, k: u32) -> u32 {
    use std::cmp;

    let mut left: u32 = 0;
    let mut curr = 0; // the current sum of the window
    let mut answer = 0; // the maximum size of the window in which the sum is equal to k

    // increment 'right' pointer across array
    for (right, value) in nums.iter().enumerate() {
        // always add the new value, to progress the window along the array
        curr += *value;
        /*
        if new value exceeds k, remove left element values until <= k
        this shrinks the window and reduces the current sum
        */
        while curr > k {
            curr -= nums[left as usize];
            left += 1;
        }

        answer = cmp::max(answer, (right as u32 - left) + 1);
    }

    return answer;
}

// find maximum length of subarray/window of NUMS where the sum of the window is equal to K
pub fn run_sum_of_window() {
    let nums = vec![3, 1, 2, 7, 4, 2, 1, 1, 5];
    let k = 8;
    let len = sum_of_window(&nums, k);
    println!("maximum length of subarray with sum {} = {}", k, len);
}

/* -- 713: Subarray Product Less Than K --
https://leetcode.com/problems/subarray-product-less-than-k/

Given an array of positive integers nums and an integer k, return the number of subarrays where the product of all the elements in the subarray is strictly less than k.

For example, given the input nums = [10, 5, 2, 6], k = 100, the answer is 8. The subarrays with products less than k are:

[10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6]
*/

pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    if k <= 1 {
        return 0;
    };

    let mut left = 0;
    let mut curr = 1;
    let mut answer: i32 = 0;

    for (right, value) in nums.iter().enumerate() {
        curr *= value;

        while curr >= k {
            curr = curr / nums[left];
            left += 1;
        }

        answer += right as i32 - left as i32 + 1;
    }

    return answer;
}

pub fn run_number_of_subarrays() {
    let k = 100;
    let nums = vec![10, 5, 2, 6];
    let number_of_subarrays = number_of_subarrays(nums, k);
    println!("num of subarrays: {}", number_of_subarrays)
}

/*
Fixed Window Size

Example 4: Given an integer array nums and an integer k, find the sum of the subarray with the largest sum whose length is k.
*/

fn fixed_window_size(nums: Vec<i32>, k: i32) -> i32 {
    use std::cmp;

    let mut curr = 0; // current sum of integers in window of size k

    for i in 0..k as usize {
        curr += nums[i];
    }

    let mut answer = curr;
    println!("current sum: {}", curr);

    // iterate over the remainder of the array, keeping the window size
    for i in k..nums.len() as i32 {
        curr += nums[i as usize];
        println!("Curr after Add right element: {} (k = {})", curr, k);
        curr -= nums[(i - k) as usize];
        println!("Curr after Subtract left element: {} (k={})", curr, k);

        answer = cmp::max(answer, curr);
    }

    return answer;
}

pub fn run_fixed_window_size() {
    let k = 4;
    let nums = vec![3, -1, 4, 12, -8, 5, 6];
    let largest_sum = fixed_window_size(nums, k);
    println!("Largest sum of subarray size: {} is {}", k, largest_sum);
}

/*
Maximum Average Subarray

You are given an integer array nums consisting of n elements, and an integer k.

Find a contiguous subarray whose length is equal to k that has the maximum average value and return this value. Any answer with a calculation error less than 10-5 will be accepted.
*/

fn maximum_avg_subarray(nums: Vec<i32>, k: i32) -> f64 {
    use std::cmp;

    if nums.len() == 1 {
        return nums[0] as f64;
    }

    let mut _answer = 0;
    let mut current = 0;
    // build first subarray of k
    for i in 0..k {
        current += nums[i as usize];
    }

    println!("Initial value of current = {}", current);
    _answer = current;

    if k == nums.len() as i32 {
        return _answer as f64 / k as f64;
    }

    for i in k..nums.len() as i32 {
        current += nums[i as usize]; // top of the subarray
        current -= nums[(i - k) as usize]; // bottom of the subarray

        _answer = cmp::max(_answer, current)
    }

    return _answer as f64 / k as f64;
}

pub fn run_maximum_avg_subarray() {
    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 4;
    let correct = 12.75000;
    let my_answer = maximum_avg_subarray(nums, k);
    println!("Test: {} == {}", correct, my_answer);
    assert_eq!(correct, my_answer);
}

/*
Max Consecutive Ones III

Given a binary array nums and an integer k, return the maximum number of consecutive 1's in the array if you can flip at most k 0's.

*/
fn maximum_consecutive_ones(nums: Vec<i32>, k: i32) -> i32 {
    // iterate through looking for the longest string of 1s.
    // if a zero is found, include it.
    // decrement when more than 1 zero is found.
    // track the largest size of array during the process.

    use std::cmp;

    let mut left = 0;
    let mut current = 0;
    let mut cur_zeros = 0; // maximum k
    let mut answer = 0;

    for (_, val) in nums.iter().enumerate() {
        if *val == 0 {
            cur_zeros += 1;
        }

        while cur_zeros > k {
            if nums[left] == 0 {
                cur_zeros -= 1;
            }
            left += 1;
            current -= 1;
        }

        current += 1;

        answer = cmp::max(answer, current);
    }

    return answer;
}

pub fn run_maximum_consecutive_ones() {
    let k = 2;
    let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
    let answer = 6;
    let my_answer = maximum_consecutive_ones(nums, k);
    println!("my answer: {}, actual: {}", my_answer, answer);
    assert_eq!(my_answer, answer);
}
