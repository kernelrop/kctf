use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

use kctf::{KctfErrors, KctfPow};
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let default_path = String::from("kctf");
    let binary_path = args.first().unwrap_or(&default_path);

    if args.len() < 3 {
        usage(&binary_path);
    }

    match args[1].as_str() {
        "solve" => solve(&args[2]),
        "ask" => ask(&args[2]),
        "gen" => gen(&args[2]),
        "verify" => {
            if args.len() != 4 {
                usage(&binary_path)
            }
            verify(&args[2], &args[3])
        }
        _ => {
            usage(&binary_path);
        }
    }
}

fn usage(binary_path: &str) -> () {
    let _ = stdout().write_all(
        format!(
            "Usage:
    Solve pow: {0} solve <challenge>
    Check pow: {0} ask <difficulty>
    Gen challenge: {0} gen <difficulty>
    Verify challenge: {0} verify <challenge> <solution>
    Get help: {0} help
",
            binary_path
        )
        .as_bytes(),
    );
    let _ = stdout().flush();
    exit(1)
}

fn print(string: &str) {
    let _ = std::io::stdout().write_all(string.as_bytes());
    let _ = stdout().flush();
}

fn print_error(error_string: &str) {
    print(error_string);
    exit(1)
}

fn print_success(success_string: &str) {
    print(success_string);
    exit(0)
}

fn generate_challenge(difficulty: &str) -> KctfPow {
    let difficulty: u32 = difficulty.parse().unwrap_or_else(|_| {
        print_error("Unable to parse difficulty\n");
        panic!("Unreachable code");
    });

    if difficulty == 0 {
        print_success("== proof-of-work: disabled ==\n");
    }

    KctfPow::gen_challenge(difficulty)
}

fn ask(difficulty: &str) {
    let challenge = generate_challenge(difficulty);

    print(&format!(
        "== proof-of-work: enabled ==
please solve a pow first
You can run the solver with:
    kctf solve {}
===================

solution?
",
        challenge.serialize_challenge()
    ));

    let mut solution = String::new();
    stdin().read_line(&mut solution).unwrap_or_else(|_| {
        print_error("Could not read from stdin\n");
        panic!("Unreachable code");
    });

    match challenge.verify(&solution.trim()) {
        Ok(true) => {
            print_success("Correct\n");
        }

        _ => {
            print_error("Proof-of-work fail\n");
        }
    };
}

fn kctf_error_handler(error: KctfErrors) {
    match error {
        KctfErrors::UnknownVersion => {
            print_error("Unknown challenge version\n");
        }
        KctfErrors::DecodeError => {
            print_error("Invalid base64 encoding\n");
        }
        KctfErrors::FormatError => {
            print_error("Wrong format\n");
        }
        KctfErrors::LargeDifficulty => {
            print_error("Difficulty too large\n");
        }
    }
}

fn solve(challenge: &str) {
    let now = Instant::now();

    let challenge = match KctfPow::from_challenge(challenge) {
        Ok(v) => v,
        Err(e) => {
            kctf_error_handler(e);
            return;
        }
    };

    let solution = challenge.solve();
    print_success(&format!(
        "Solved challenge in {:.5?}: {}\n",
        now.elapsed(),
        solution
    ));
}

fn verify(challenge: &str, solution: &str) {
    let now = Instant::now();
    let challenge = match KctfPow::from_challenge(challenge) {
        Ok(v) => v,
        Err(e) => {
            kctf_error_handler(e);
            return;
        }
    };

    match challenge.verify(solution) {
        Ok(true) => print_success(&format!(
            "Verified challenge is correct in {:.5?}\n",
            now.elapsed()
        )),
        Ok(false) => print_error(&format!(
            "Verified challenge is incorrect in {:.5?}\n",
            now.elapsed()
        )),
        Err(e) => {
            kctf_error_handler(e);
            return;
        }
    };
}

fn gen(difficulty: &str) {
    let _ = stdout().write_all(
        format!(
            "Challenge: {}\n",
            generate_challenge(difficulty).serialize_challenge()
        )
        .as_bytes(),
    );
    let _ = stdout().flush();
    exit(0)
}
