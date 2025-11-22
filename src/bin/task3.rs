use std::collections::HashMap;
use std::io;

fn main() {
    let mut phonebook: HashMap<String, String> = HashMap::new();
    
    loop {
        println!("\n1.저장 2.검색 3.종료");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => {
                println!("이름:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                println!("번호:");
                let mut num = String::new();
                io::stdin().read_line(&mut num).unwrap();
                phonebook.insert(name.trim().to_string(), num.trim().to_string());
                println!("저장완료!");
            },
            "2" => {
                println!("검색할 이름:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                match phonebook.get(name.trim()) {
                    Some(n) => println!("번호: {}", n),
                    None => println!("없습니다."),
                }
            },
            "3" => break,
            _ => continue,
        }
    }
}
