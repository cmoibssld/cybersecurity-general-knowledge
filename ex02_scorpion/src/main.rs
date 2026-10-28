use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Param {
    file: Vec<String>,
}

fn main() {
    let param = Param::parse();
    println!("Hello, world!: {:?}", param.file);
}
