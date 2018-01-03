use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

fn main() {
    let path = std::env::args().nth(1).expect("no input file given!");
    let f = File::open(path).expect("unable to open file");
    let f = BufReader::new(f);
    let mut memmory: Vec<usize> = f.lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut known_configurations = HashMap::new();
    let mut step = 0;
    let loop_len;
    loop {
        //println!("{:?}", memmory);
        step += 1;

        // sadly, max_by_key returns the last match, not the first => manual search ;__;
        let mut max = 0usize;
        let mut max_pos = 0usize;
        for (i, &x) in memmory.iter().enumerate() {
            if x > max {
                max = x;
                max_pos = i;
            }
        }
        memmory[max_pos] = 0;
        for i in 0usize..max {
            let index = (max_pos + 1 + i) % memmory.len();
            memmory[index] += 1;
        }

        match known_configurations.insert(memmory.clone(), step) {
            Some(last_step) => {
                loop_len = step - last_step;
                break;
            }
            None => {}
        }
    }
    println!("{} steps, loop length: {}", step, loop_len);
}
