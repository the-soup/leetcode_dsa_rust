#![allow(dead_code)]

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = nums.len() - 1;

    while i < nums.len() {
        i = if i > (nums.len() - 1) {
            nums.len() - 1
        } else {
            i
        };
        let sum = nums[i] + nums[j];
        println!("sum {} i = {} j = {}", sum, i, j);

        if sum == target {
            return vec![i as i32, j as i32];
        } else if sum < target {
            i += 1;
        } else {
            if j != 0 {
                j -= 1;
            }
        }
    }

    return vec![];
}

pub fn run_two_sum() {
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;
    let ans = vec![0, 1];
    let result = two_sum(nums1, target1);
    assert_eq!(ans, result);
}
