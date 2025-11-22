use std::fs;
use std::path::Path;

fn visit_dirs(dir: &Path, cb: &dyn Fn(&std::fs::DirEntry)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn main() {
    println!("현재 디렉토리의 모든 파일을 나열합니다 (ls -R 효과):");
    visit_dirs(Path::new("."), &|entry| {
        println!("{:?}", entry.path());
    }).unwrap();
}
