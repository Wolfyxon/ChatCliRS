use std::io::{stdin, stdout, Write};

pub fn input() -> String {
    
    let mut res = String::new();
    
    stdout().flush().expect("Failed to flush stdout");
    stdin().read_line(&mut res).expect("Failed to read line from stdin");
    
    if let Some('\n') = res.chars().next_back() {
        res.pop();
    }

    if let Some('\r') = res.chars().next_back() {
        res.pop();
    }

    res
}