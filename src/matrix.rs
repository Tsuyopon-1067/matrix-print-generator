use rand::Rng;

// 行列の構造体
#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<i32>>,
}

impl Matrix {
    // 新しい行列を生成
    pub fn new(rows: usize, cols: usize, data: Vec<Vec<i32>>) -> Self {
        Self { rows, cols, data }
    }

    // 乱数で新しい行列を生成
    pub fn new_random(rows: usize, cols: usize, min_val: i32, max_val: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut data = Vec::with_capacity(rows);
        for _ in 0..rows {
            let row = (0..cols)
                .map(|_| rng.gen_range(min_val..=max_val))
                .collect();
            data.push(row);
        }
        Self { rows, cols, data }
    }

    // Typst形式の文字列に変換
    pub fn to_typst(&self) -> String {
        let mut s = "$mat(".to_string();
        for (i, row) in self.data.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                s.push_str(&val.to_string());
                if j < self.cols - 1 {
                    s.push_str(", ");
                }
            }
            if i < self.rows - 1 {
                s.push_str("; ");
            }
        }
        s.push_str(")$");
        s
    }
}
