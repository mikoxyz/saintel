use sainte_lague::distribute;
use std::io;
use std::io::Write;

fn get_user_input(string_out: &str) -> String {
    let mut input = String::new();
    while input.trim().is_empty() {
        if !string_out.is_empty() {
            print_flush(string_out);
        }

        io::stdin().read_line(&mut input).expect("err");
    }

    input
}

/*
 * party_count_init and seats_init could probably be turned into a single macro, idk i don't know
 * rust
 */
fn party_count_init() -> usize {
    loop {
        if let Ok(i) = get_user_input("Parties: ").trim().parse::<usize>() {
            if i > 0 {
                return i;
            }
        }
    }
}

fn print_flush(string_out: &str) {
    print!("{}", string_out);

    io::stdout().flush().expect("err");
}

fn seats_init() -> usize {
    loop {
        if let Ok(i) = get_user_input("Seats: ").trim().parse::<usize>() {
            if i != 0 {
                return i;
            }
        }
    }
}

fn vote_counts_init(party_count: usize) -> Vec<f64> {
    let mut vote_counts = Vec::new();

    for n in 0..party_count {
        loop {
            if let Ok(i) = get_user_input(format!("Party {} votes: ", n + 1).as_str())
                .replace(',', "")
                .trim()
                .parse()
            {
                if i >= 0.0 {
                    vote_counts.push(i);
                    break;
                }
            }
        }
    }

    vote_counts
}

fn main() {
    let party_count = party_count_init();
    let draw_on_tie = matches!(
        get_user_input("Draw on tie? ").trim(),
        "yes" | "y" | "true" | "t"
    );

    let distribution = distribute(&vote_counts_init(party_count), &seats_init(), &draw_on_tie).expect("err: ");

    print_flush("\n");
    for n in 0..party_count {
        println!("Party {}: {} seats", n + 1, distribution[n]);
    }
}
