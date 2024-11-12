mod cli;
mod vars;
mod token;
mod syntax;
mod gap_buffer;

use clap::Parser;
use crossterm::style::{Color, Colors, Print, SetColors, Stylize};
use std::io::{stdout, Write};
use crossterm::{QueueableCommand};

pub use vars::*;

// pub const GAP_BUFFER_EXTRA_LEN: usize = 64;

// pub struct Context {
//     // pub stdout
// }

// fn setup() {
//     // let mut stdout = stdout();
//     execute!(
//          std::io::stdout(),
//          EnableBracketedPaste,
//          EnableFocusChange,
//          EnableMouseCapture
//     )?;
// }

use gap_buffer::GapBuffer;

fn main() {
    let mut buf = GapBuffer::new("Hello World!".to_string());
    println!("{:?}", buf);

    buf.move_to(6);

    println!("{:?}", buf);

    buf.move_to(5);

    println!("{:?}", buf);

    buf.push_char('@');

    println!("{:?}", buf);

    // let mut buf = GapBuffer::new("Hello there!");
    // println!("got: {:?}", buf.to_string());
    // buf.insert('c');
    // println!("got: {:?}", buf.to_string());
    // // buf
    // // force compile regex
    // LazyLock::force(&RE);

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
