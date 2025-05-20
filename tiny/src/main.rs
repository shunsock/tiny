use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", format_args!("{:?}", args));
}
