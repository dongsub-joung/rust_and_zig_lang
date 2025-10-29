use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buff= std.lock().lines();
    let input_v: Vec<usize>= buff.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<usize>().unwrap())
        .collect();

    let n= input_v[0];
    let m= input_v[1];

    let mut idx= 0;
    while idx < m {
        println!("OK");
        idx+=1;
    }

    loop {
        println!("Too Many Requests");
        if idx > m {
            return;
        }
        idx+=1;
    }
}