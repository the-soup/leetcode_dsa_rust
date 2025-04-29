#![allow(dead_code)]

fn find_nemo(array: Vec<&str>) {
    for item in array {
        if item == "nemo" {
            println!("Found Nemo!")
        } else {
            println!("Not Nemo: {}!", item);
        }
    }
}

fn run_find_nemo() {
    let _nemo = vec!["nemo"];
    let _everyone = vec![
        "dory", "bruce", "marlin", "nemo", "gill", "bloat", "nigel", "squirt", "darla", "hank",
    ];

    let large = vec!["nemo"; 100000];

    find_nemo(large);
}
