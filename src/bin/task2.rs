use std::io;

fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("입력 실패");
    buffer
}

fn main() {
    println!("=== 행렬 덧셈 프로그램 ===");
    println!("행과 열의 크기를 입력하세요 (예: 2 2):");
    
    let input = get_input();
    let dims: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse().unwrap_or(0)).collect();
        
    if dims.len() < 2 { println!("잘못된 입력입니다."); return; }
    let (rows, cols) = (dims[0], dims[1]);

    println!("첫 번째 행렬 입력 ({}줄):", rows);
    let mat1 = read_matrix(rows, cols);
    
    println!("두 번째 행렬 입력 ({}줄):", rows);
    let mat2 = read_matrix(rows, cols);

    println!("=== 결과 행렬 ===");
    for i in 0..rows {
        for j in 0..cols {
            print!("{} ", mat1[i][j] + mat2[i][j]);
        }
        println!();
    }
}

fn read_matrix(rows: usize, cols: usize) -> Vec<Vec<i32>> {
    let mut matrix = Vec::new();
    for _ in 0..rows {
        let input = get_input();
        let row: Vec<i32> = input.split_whitespace()
            .map(|s| s.parse().unwrap_or(0)).collect();
        if row.len() == cols { matrix.push(row); }
        else { matrix.push(vec![0; cols]); } // 에러 방지용 0 채움
    }
    matrix
}
