#![allow(dead_code)]

fn merge_sorted_arrays(first: &Vec<i32>, second: &Vec<i32>) -> Vec<i32> {
    let mut output = vec![];
    let mut i = 0;
    let mut j = 0;

    // Validate input
    if first.len() == 0 {
        return second.clone();
    };
    if second.len() == 0 {
        return first.clone();
    };

    // Compare array element
    while (i < first.len()) || (j < second.len()) {
        if i == first.len() {
            // dont compare array values
            output.push(second[j]);
            j += 1;
            continue;
        }

        if j == second.len() {
            output.push(first[i]);
            i += 1;
            continue;
        }

        if first[i] < second[j] {
            output.push(first[i]);
            i += 1;
        } else {
            output.push(second[j]);
            j += 1;
        }

        println!("{:?}", output);
    }

    return output;
}

pub fn run_merge_sorted_arrays() {
    let set1 = vec![0, 3, 4, 31];
    let set2 = vec![4, 6, 30];
    let final_set = vec![0, 3, 4, 4, 6, 30, 31];

    let merged_set = merge_sorted_arrays(&set1, &set2);
    println!("merged set \n\t {:?}", merged_set);
    assert_eq!(final_set, merged_set, "Sets do not match");
}
