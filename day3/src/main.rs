/*
 * 17 16 15 14 13
 * 18  5  4  3 12  x  -
 * 19  6  1  2 11 28  |              -
 * 20  7  8  9 10 27  | offset_side  | ring - 1
 * 21 22 23 24 25 26  -              -
 *        |--------|
 *          ring+1
 */

fn manhattan(input: &u32) -> u32 {
    let ring = (((input - 1) as f64).sqrt() as u32 + 1) / 2;
    let offset = input - (2 * ring - 1).pow(2) - 1;
    let ring_size = (2 * ring + 1).pow(2) - (2 * ring - 1).pow(2);
    let offset_side = offset % (ring_size / 4);
    let distance = ring + ((offset_side as i32 - ring as i32 + 1).abs() as u32);
    println!("{} {} {}", ring, offset_side, distance);
    distance
}

fn stress_test(input: &u32) -> u32 {
    let ring = ((((input - 1) as f64).sqrt() as u32 + 1) / 2) as usize;
    let diameter = (2 * ring + 1) as usize; // TODO: better heuristic for needed size, this is way to large
    let mut memory = vec![vec![0; diameter]; diameter]; // [y][x]

    memory[ring][ring] = 1;

    let mut position: (usize, usize) = (ring, ring); // start in center
    let mut direction = 0;
    let mut step = 1;
    let mut edge_position = 0;
    let mut val;
    loop {
        val = 0;
        for dx in [-1, 0, 1].iter() {
            for dy in [-1, 0, 1].iter() {
                val += memory[(position.0 as i32 + dy) as usize][(position.1 as i32 + dx) as usize];
            }
        }
        memory[position.0][position.1] = val;

        edge_position += 1;
        match direction {
            0 => {
                position.1 += 1;
            }
            1 => {
                position.0 += 1;
            }
            2 => {
                position.1 -= 1;
            }
            3 => {
                position.0 -= 1;
            }
            _ => {}
        }

        if edge_position >= step {
            edge_position = 0;
            direction = (direction + 1) % 4;
            if direction == 0 || direction == 2 {
                step += 1;
            }
        }

        if val > *input {
            break;
        }
    }
    val
}

fn main() {
    let mut args = std::env::args();
    let input: u32 = args.nth(1)
        .expect("please provide an input!")
        .parse()
        .expect("please provide a number!");
    let output = manhattan(&input);
    let output2 = stress_test(&input);
    println!("{}\n{}", output, output2);
}
