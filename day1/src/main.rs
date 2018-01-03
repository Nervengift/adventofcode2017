fn inverse_captcha(input: &String) -> u32 {
    let l = input
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut last = l.last().unwrap();
    let mut sum = 0;
    for x in l.iter() {
        if last == x {
            sum += x
        }
        last = x;
    }

    return sum;
}

fn inverse_captcha2(input: &String) -> u32 {
    let l = input
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let len = l.len();

    let mut sum = 0;
    for (i, x) in l.iter().enumerate() {
        let other = l[(i + len / 2) % len];
        if other == *x {
            sum += *x
        }
    }

    return sum;
}

fn main() {
    let mut args = std::env::args();
    let input = args.nth(1).unwrap_or_else(|| {
        println!("please provide an input!");
        std::process::exit(1);
    });
    let output = inverse_captcha(&input);
    let output2 = inverse_captcha2(&input);
    println!("{}\n{}", output, output2);
}
