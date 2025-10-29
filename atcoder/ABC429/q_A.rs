use std::{io::{stdin, BufRead}, process::id};
use std::io::{self, Write};

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
    while idx < n && idx < m {
        let ok= "OK".trim();
        print!("{}", ok);
        idx+=1;
    }

    if n < m {
        return;
    }

    loop {
        let out_of_range= "Too Many Requests".trim();
        print!("{}", out_of_range);
        if idx > m {
            return;
        }
        idx+=1;
    }
}