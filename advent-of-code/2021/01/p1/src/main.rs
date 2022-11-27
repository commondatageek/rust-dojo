use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut lines = read_lines("../input.txt")
        .expect("could not read lines from ../input.txt");

    let mut increased_ct = 0;

    // preload "prev" with the first line
    let mut prev = lines.next()
        .expect("input file has no lines")
        .unwrap();

    for curr in lines {
        let curr = curr.unwrap();
        if increased(&prev, &curr) {
            increased_ct += 1;
        }
        prev = curr;
    }

    println!("total increase: {increased_ct}");
}

fn increased(prev: &String, curr: &String) -> bool {
    let prev = prev.parse::<i32>().unwrap();
    let curr = curr.parse::<i32>().unwrap();

    curr > prev
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}