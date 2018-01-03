use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn row1(numbers: &Vec<u32>) -> u32 {
    let max = numbers.iter().max().unwrap();
    let min = numbers.iter().min().unwrap();
    return max - min;
}

fn row2(numbers: &Vec<u32>) -> u32 {
    for x in numbers.iter() {
        match numbers.iter().find(|y| x % *y == 0 && x != *y) {
            Some(y) => {
                return x / y;
            }
            None => {}
        }
    }
    return 0;
}

fn main() {
    let path = std::env::args().nth(1).expect("no input file given!");
    let f = File::open(path).expect("unable to open file");
    let f = BufReader::new(f);

    let mut sum1 = 0u32;
    let mut sum2 = 0u32;
    for line in f.lines() {
        let line = line.unwrap();
        let numbers: Vec<u32> = line.split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        sum1 += row1(&numbers);
        sum2 += row2(&numbers);
    }
    print!("{}\n{}\n", sum1, sum2);
}
