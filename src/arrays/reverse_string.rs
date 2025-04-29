#![allow(dead_code)]
fn reverse_string(input: &String) -> String {
    match input.len() {
        0 => return String::from("Empty string"),
        1 => return input.to_owned(),
        _ => {
            // let mut output = String::default();

            // OLD WAY
            // for letter in input.chars().into_iter().rev() {
            //     output.push(letter);
            // }
            // return output

            // More idiomatic
            return input.chars().into_iter().rev().collect();
        }
    }
}

pub fn run_reverse_string() {
    let my_string: String = String::from("Hi My name is Andrei");
    let rev_string: String = String::from("ierdnA si eman yM iH");

    println!("Reverse the string: {}", my_string);
    let my_rev_string = reverse_string(&my_string);
    println!("{}", my_rev_string);
    println!("String reversed? {}", (rev_string == my_rev_string));
}
