#![allow(dead_code)]

use std::fs;
use std::io::Write;
use std::process::Command;
use std::env;
use std::path::Path;

fn equal(s: &str, v: Vec<u8>) -> bool {
    if s.len() != v.len() {
        return false;
    }
    for i in 0..s.len() {
        if s.as_bytes()[i] != v[i] {
            return false;
        }
    }
    return true;
}

#[derive(Clone, Copy)]
enum Token {
    Right,
    Left,
    Plus,
    Minus,
    Out,
    In,
    Loop,
    ELoop,
}

fn main() {
    if env::args().len() == 1 {
        println!("Usage:\tbf <filename>");
        println!("\tbf <path/to/file.bf>");
        return;
    }
    println!("{}", env::consts::OS);
    if env::consts::OS == "linux" {
        let out = Command::new("sh")
                                    .arg("--installed")
                                    .arg("gcc")
                                    .arg("apt list")
                                    .output()
                                    .expect("Cannot check for compiler. Make sure gcc is installed before usage");
        let text = out.stdout;
        if equal("Listing...\n", text) {
            println!("It seems like you don't have gcc installed");
            println!("sudo apt-get install gcc should work on most machines");
            return;
        }
        println!("checked gcc");
    }
    let f = env::args().nth(1).unwrap();
    assert!(Path::new(f.as_str()).exists(), "It seems like the provided file does not exist");
    let cont = fs::read_to_string(f.clone()).expect("Cannot access filesystem");
    let toks = token(cont);
    let mut s: String = "".to_string();
    for tok in toks.clone() {
        match tok {
            Token::Right => s.push_str("RIGHT\n"),
            Token::Left  => s.push_str("LEFT\n"),
            Token::Plus  => s.push_str("PLUS\n"),
            Token::Minus => s.push_str("MINUS\n"),
            Token::Out   => s.push_str("OUT\n"),
            Token::In    => s.push_str("IN\n"),
            Token::Loop  => s.push_str("LOOP\n"),
            Token::ELoop => s.push_str("ENDLOOP\n"),
        }
    }
    let c = to_c(toks);
    let mut file = fs::File::create(Path::new(f.clone().as_str()).with_extension("c")).expect("Cannot access filesystem");
    file.write_all(c.as_bytes()).expect("Cannot write to file");
    if env::consts::OS == "linux" {
        let _ = Command::new("gcc")
                .arg(Path::new(f.clone().as_str()).with_extension("c").to_str().expect("Cannot access filesystem"))
                .arg("-o")
                .arg(Path::new(f.clone().as_str()).with_extension("").to_str().expect("Cannot access filesystem"))
                .output()
                .expect("Cannot compile make sure the compiler is installed and in PATH");
        //fs::remove_file(Path::new(f.clone().as_str()).with_extension("c").to_str().expect("Cannot access filesystem")).expect("Cannot access filesystem");
        println!("compiled");
    } else if env::consts::OS == "windows" {
        let _ = Command::new("gcc")
                .arg(Path::new(f.clone().as_str()).with_extension("c").to_str().expect("Cannot access filesystem"))
                .arg("-o")
                .arg(Path::new(f.clone().as_str()).with_extension("exe").to_str().expect("Cannot access filesystem"))
                .output()
                .expect("Cannot compile make sure the compiler is installed and in PATH");
        fs::remove_file(Path::new(f.clone().as_str()).with_extension("c").to_str().expect("Cannot access filesystem")).expect("Cannot access filesystem");
    }
}

fn parse(s: String) -> bool {
    let mut open: u8 = 0;
    for i in s.as_bytes() {
        match i.to_ascii_lowercase() as char {
            '[' => open += 1,
            ']' => {
                if open > 0 {
                    open -= 1;
                } else {
                    return false;
                }
            }
            '<' | '>' | '+' | '-' | '.' | ',' => (),
            _ => (),
        }
    }

    return true;
}

fn token(s: String) -> Vec<Token> {
    assert!(parse(s.clone()), "The loops aren't closed properly");
    let mut res: Vec<Token> = vec![];
    for i in s.clone().chars() {
        match i {
            '>' => res.push(Token::Right),
            '<' => res.push(Token::Left),
            '+' => res.push(Token::Plus),
            '-' => res.push(Token::Minus),
            '.' => res.push(Token::Out),
            ',' => res.push(Token::In),
            '[' => res.push(Token::Loop),
            ']' => res.push(Token::ELoop),
            _ => (),
        }
    }

    return res;
}

fn to_c(v: Vec<Token>) -> String {
    let mut s: String = "#include <stdio.h>\nunsigned char t[30000];\nshort p = 0;\nint main() {\n".to_string();
    for i in v {
        match i {
            Token::Right => s.push_str("p++;\n"),
            Token::Left  => s.push_str("p--;\n"),
            Token::Plus  => s.push_str("t[p]++;\n"),
            Token::Minus => s.push_str("t[p]--;\n"),
            Token::Out   => s.push_str("putchar(t[p]);\n"),
            Token::In    => s.push_str("t[p] = (char)(getchar() % 256);\n"),
            Token::Loop  => s.push_str("while (t[p] > 0) {\n"),
            Token::ELoop => s.push_str("}\n"),
        }
    }
    s.push_str("return 0;\n}");

    return s;
}