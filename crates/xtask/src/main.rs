use clap::Parser;

fn main() {
    let cmd = Cmd::parse();

    todo!("Run {cmd:?}");
}

#[derive(Parser, Debug)]
#[command(author, version)]
enum Cmd {}
