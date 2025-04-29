fn climb_stairs(step_index: usize, n_stairs: u32, memo: &mut Vec<u32>) -> u32 {
    if step_index as u32 > n_stairs {
        return 0;
    }

    if step_index as u32 == n_stairs {
        return 1;
    }

    if memo[step_index] > 0 {
        return memo[step_index];
    }

    memo[step_index] =
        climb_stairs(step_index + 1, n_stairs, memo) + climb_stairs(step_index + 2, n_stairs, memo);
    return memo[step_index];
}

fn calculate_climb_stairs(n_stairs: u32) -> u32 {
    let mut memoi: Vec<u32> = vec![0; (n_stairs + 1) as usize];
    return climb_stairs(0, n_stairs, &mut memoi);
}

pub fn run_climbing_stairs() {
    let stairs = 3;
    let answer = 3;
    let ways = calculate_climb_stairs(stairs);
    println!("calculated: {} answer: {}", ways, answer);
    assert_eq!(ways, answer);
}
