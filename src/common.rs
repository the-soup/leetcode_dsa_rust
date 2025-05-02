pub fn evaluate_vec_i32(input: Vec<i32>, output: i32, my_func: fn(Vec<i32>) -> i32) {
    let my_answer = my_func(input);
    println!("my answer: {}, \nanswer: {}", my_answer, output);
    assert_eq!(my_answer, output);
    println!("Correct!")
}

pub fn evaluate_nested_vec_i32(input: Vec<Vec<i32>>, output: Vec<i32>, my_func: fn(Vec<Vec<i32>>) -> Vec<i32>) {
    let my_answer = my_func(input);
    println!("my answer: {:?}, \nanswer: {:?}", my_answer, output);
    assert_eq!(my_answer, output);
    println!("Correct!")
}