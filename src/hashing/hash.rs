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
