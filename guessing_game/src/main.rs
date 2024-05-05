use rand::{thread_rng, Rng};

fn main() {
    let secret: u32 = generate_secret();
    compare(secret);
}

fn compare(secret: u32) {
    loop {
        let guess: u32 = user_input();
        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Less than secret\n"),
            std::cmp::Ordering::Greater => println!("Greater than secret\n"),
            std::cmp::Ordering::Equal => {
                println!("Equal to secret");
                break;
            }
        }
    }
}

fn generate_secret() -> u32 {
    let secret: u32 = thread_rng().gen_range(1..=100);
    return secret;
}

fn user_input() -> u32 {
    let mut guess: String = String::from("");

    println!("Hello player please enter your guess");
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Not a valid guess");

    let guess: u32 = guess.trim().parse().expect("Not a valid number");
    println!("Your guess is {guess}");

    return guess;
}
