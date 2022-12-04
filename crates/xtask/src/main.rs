use clap::Parser;

fn main() {
    match Cmd::parse() {}
}

#[derive(Parser, Debug)]
#[command(author, version)]
enum Cmd {
    // TODO: add some arguments
}
