
use std::{env, fs::File, io::Read};
use MagiInterpreter::interpreter::MagiInterpreter;
use antlr_rust::tree::ParseTreeVisitorCompat;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut input = String::new();

    if args.len() > 1 {
        let path = args.get(1).unwrap();
        let mut file_in = File::create_new(path).expect("Failed to open path");
        file_in.read_to_string(&mut input).expect("Failed to read the file");
    } else {
        println!("Enter a string to parse:");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }

    println!("CODE: \n{}", input);

    let tree = MagiInterpreter::parse(&input);
    let mut interpreter = MagiInterpreter::new();
    interpreter.visit(&*tree);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignment() {

    }
}
