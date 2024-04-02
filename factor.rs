use std::env;
use std::process::ExitCode;

fn min_div(n: i64) -> i64 {
    let n0: i64 = n;
    let mut divisor = 2;

    while n0 != 1 {
        if n0 % divisor == 0 {
            break;
        }

        divisor += 1;
    }

    divisor
}

fn factor(n: i64) -> Vec<i64> {
    let mut v = vec!();
    let mut n0: i64 = n;

    while n0 != 1 {
        let divisor = min_div(n0);
        v.push(divisor);
        n0 /= divisor;
    }

    v
}

fn show_factors(n: i64) {
    let factors = factor(n);

    print!("{}: ", n);
    for f in &factors {
        print!("{} ", f);
    }
    println!();
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    if args.len() < 2 {
        println!("{}: missing number to factorize", program);
        return ExitCode::FAILURE;
    }

    let number = &args[1];
    let parsed = number.parse::<i64>();
    match parsed {
        Ok(_) => {
            let n = parsed.unwrap();
            if n < 0 {
                println!("{}: {} is not a valid positive integer", program, n);
                return ExitCode::FAILURE;
            }
            show_factors(n);
        },
        _ => {
            println!("{}: '{}' is not a valid number", program, number);
            return ExitCode::FAILURE;
        },
    };

    ExitCode::SUCCESS
}
