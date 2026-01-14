use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: wc [FILE]");
        process::exit(1);
    }

    let filename = &args[1];

    let bytes = match fs::read(filename) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("wc: {}: {}", filename, e);
            process::exit(1);
        }
    };

    let byte_count = bytes.len();

    let mut line_count = 0;
    let mut word_count = 0;
    let mut in_word = false;

    for &b in &bytes {
        if b == b'\n' {
            line_count += 1;
        }

        if b.is_ascii_whitespace() {
            in_word = false;
        } else if !in_word {
            word_count += 1;
            in_word = true;
        }
    }

    println!(
        "{:>4} {:>4} {:>4} {}",
        line_count, word_count, byte_count, filename
    );
}

