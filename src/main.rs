use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <arguments>", args[0]);
        process::exit(1);
    }

    println!("{}", args[1..].join(" "));  // formatではなくprintlnを使用
}