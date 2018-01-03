use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn jump_around(jumps: Vec<i32>, part2: bool) -> usize {
    let mut jumps = jumps;
    let max = jumps.len() as i32 - 1;

    let mut position: i32 = 0;
    let mut steps: usize = 0;
    loop {
        //println!("{:?}: {}", jumps, position);
        let jump = jumps[position as usize];
        jumps[position as usize] += if jump >= 3 && part2 { -1 } else { 1 };
        position = position + jump;
        steps += 1;

        if position < 0 || position > max {
            break;
        }
    }
    steps
}

fn main() {
    let path = std::env::args().nth(1).expect("no input file given!");
    let f = File::open(path).expect("unable to open file");
    let f = BufReader::new(f);

    let jumps: Vec<i32> = f.lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect();
    let num_jumps1 = jump_around(jumps.clone(), false);
    let num_jumps2 = jump_around(jumps, true);

    println!("{}\n{}", num_jumps1, num_jumps2);
}
