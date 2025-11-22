use std::io;
use std::time::Instant;

fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog";
    println!("=== 타자 연습 ===");
    println!("문장: {}", sentence);
    println!("준비되면 엔터를 누르세요.");
    
    let mut dump = String::new();
    io::stdin().read_line(&mut dump).unwrap();

    let start = Instant::now();
    println!("시작! 아래에 따라 치세요:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let duration = start.elapsed().as_secs_f64();
    let input = input.trim();
    
    // 오타 계산
    let mut errors = 0;
    let s_chars: Vec<char> = sentence.chars().collect();
    let i_chars: Vec<char> = input.chars().collect();
    
    for i in 0..std::cmp::min(s_chars.len(), i_chars.len()) {
        if s_chars[i] != i_chars[i] { errors += 1; }
    }
    
    // WPM 계산 (단어수 / 분)
    let wpm = (input.split_whitespace().count() as f64) / (duration / 60.0);
    
    println!("\n걸린 시간: {:.2}초", duration);
    println!("오타 수: {}", errors);
    println!("분당 타자수(WPM): {:.0}", wpm);
}
