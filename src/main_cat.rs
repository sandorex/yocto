//! Contains whole `yoctocat` demo program
mod vars;
mod token;
mod regex_builder;

use crossterm::style::{Color, Colors, Print, SetColors};
use crossterm::QueueableCommand;
use std::{io::{Read, Write}, path::PathBuf};
use clap::Parser;
use token::{Token, TokenType};

pub use vars::*;


/// Small program like `cat` for printing files with same experimental syntax highlighting as yocto
///
/// This binary is more for testing than actual usage
#[derive(Parser, Debug)]
#[command(name = env!("CARGO_BIN_NAME"), author, version = crate::FULL_VERSION, about)]
pub struct Cli {
    /// Path to file to read, use '-' to read from stdin
    pub path: PathBuf,
}

fn main() {
    let cli_args = Cli::parse();

    // TODO remove unwraps
    let input = if cli_args.path.as_os_str() == "-" {
        // read from stdin
        let mut stdin = std::io::stdin();
        let mut buf: Vec<u8> = Vec::new();

        stdin.read_to_end(&mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    } else {
        // read the file
        std::fs::read_to_string(&cli_args.path).unwrap()
    };

    let mut stdout = std::io::stdout();

    let tokens: Vec<Token> = regex_builder::RE.captures_iter(&input).map(|c| Token::new(&c)).collect();
    println!("{:#?}", tokens);

    for token in tokens {
        // TODO add underline etc
        let color = match token.token_type {
            TokenType::Keyword => Colors::new(Color::DarkGreen, Color::Reset),
            TokenType::Identifier => Colors::new(Color::DarkRed, Color::Reset),
            TokenType::String | TokenType::Number => Colors::new(Color::DarkMagenta, Color::Reset),
            TokenType::Symbol if token.raw == ";" => Colors::new(Color::DarkBlue, Color::Reset),
            // TokenType::Whitespace => Colors::new(Color::Reset, Color::Rgb { r: 255, g: 200, b: 200 }),
            _ => Colors::new(Color::Reset, Color::Reset),
        };

        stdout.queue(SetColors(color)).unwrap();
        stdout.queue(Print(token.raw)).unwrap();
    }

    stdout.flush().unwrap();
}
