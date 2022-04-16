use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message1 = String::from("Hello Enterprise");
    let width1 = message1.chars().count();
    let message2 = String::from("Hello Voyager");
    let width2 = message2.chars().count();
    
    let mut writer = BufWriter::new(stdout.lock());
    println!("{:#?}, {:#?}", width2, width1);
    println!("{:#?}", say(message2.as_bytes(), width2, &mut writer));
    say(message1.as_bytes(), 100, &mut writer).unwrap();
    println!("Hello, world!");
}
