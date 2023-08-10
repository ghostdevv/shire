use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {}

fn main() {
    println!("Hello, world!");
}
