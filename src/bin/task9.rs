use std::sync::mpsc;
use std::thread;
use std::io::{self, Write};

fn main() {
    // 파이프(채널) 2개 생성
    let (tx1, rx1) = mpsc::channel::<String>();
    let (tx2, rx2) = mpsc::channel::<String>();

    println!("=== 파이프 채팅 (스레드간 통신) ===");

    // 상대방 스레드
    thread::spawn(move || {
        loop {
            if let Ok(msg) = rx1.recv() {
                if msg == "exit" { break; }
                // 받은 말에 'Echo' 붙여서 돌려줌
                tx2.send(format!("상대방: '{}' 잘 들었어!", msg)).unwrap();
            }
        }
    });

    loop {
        print!("나: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();
        
        if input == "exit" { break; }
        
        tx1.send(input).unwrap(); // 전송
        
        if let Ok(reply) = rx2.recv() { // 수신
            println!("{}", reply);
        }
    }
}
