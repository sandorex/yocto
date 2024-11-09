mod cli;
mod vars;
mod token;
mod regex_builder;

use clap::Parser;
use crossterm::style::{Color, Colors, Print, SetColors};
use std::io::{Write, stdout};
use crossterm::{QueueableCommand};

pub use vars::*;

fn main() {
    // // force compile regex
    // LazyLock::force(&RE);

    let input2 = r#"// C Program to Check Prime Number using Simple Trial
// Division Approach
#include <stdio.h>

int isPrime(int N) {

    // Check divisibility from 2 to N-1
    for (int i = 2; i < N; i++) {

        // If N is divisible by i, it is not a prime number
        if (N % i == 0) {
            return 0;
        }
    }

    // If no divisors were found, N is a prime number
    return 1;
}

int main() {
    int N = 10;
    printf("Is %d prime?\n", N);

    // Check if the number is prime
    if (isPrime(N)) {
        printf("Yes\n");
    }
    else {
        printf("No\n");
    }

    return 0;
}"#;

    let cli_args = cli::Cli::parse();

    // let mut stdout = stdout();
    //
    // let tokens: Vec<Token> = RE.captures_iter(&cli_args.input).map(|c| Token::new(&c)).collect();
    // println!("{:#?}", tokens);
    //
    // for token in tokens {
    //     let color = match token.token_type {
    //         TokenType::Keyword => Colors::new(Color::Green, Color::Reset),
    //         TokenType::Identifier => Colors::new(Color::Red, Color::Reset),
    //         _ => Colors::new(Color::Reset, Color::Reset),
    //     };
    //
    //     stdout.queue(SetColors(color)).unwrap();
    //     stdout.queue(Print(token.raw)).unwrap();
    // }
    //
    // stdout.flush().unwrap();
    // println!();
}
