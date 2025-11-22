use memmap2::Mmap;
use std::fs::OpenOptions;
use std::io::Write;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("사용법: cargo run --bin task11 <원본파일> <복사본이름>");
        return Ok(());
    }

    let src_path = &args[1];
    let dst_path = &args[2];

    // 원본 열기 및 mmap
    let file = std::fs::File::open(src_path)?;
    let mmap = unsafe { Mmap::map(&file)? };

    // 새 파일 만들기
    let mut dst_file = OpenOptions::new()
        .write(true).create(true).truncate(true).open(dst_path)?;

    // 메모리 내용을 그대로 파일에 쓰기
    dst_file.write_all(&mmap)?;

    println!("mmap 복사 완료: {} -> {}", src_path, dst_path);
    Ok(())
}

