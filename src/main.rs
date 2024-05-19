use std::env::args_os;

fn main() {
    let text = args_os().nth(1).expect("You should provide a text");
    let output = args_os().nth(2).expect("You should provide an output file");
    println!("text: {:?}, output: {:?}", text, output);
}
