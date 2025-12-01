use rand::rngs::StdRng;

use crate::matrix::Matrix;

// 計算の種類
#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Sub,
    Mul,
}

// 問題の構造体
#[derive(Debug)]
pub struct Problem {
    pub operands: Vec<Matrix>,
    pub answer: Matrix,
    pub op: Operation,
}

impl Problem {
    // 新しい足し算問題を生成
    pub fn new_add(rows: usize, cols: usize, min_val: i32, max_val: i32, rng: &mut StdRng) -> Self {
        let m1 = Matrix::new_random(rows, cols, min_val, max_val, rng);
        let m2 = Matrix::new_random(rows, cols, min_val, max_val, rng);
        let mut answer_data = vec![vec![0; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                answer_data[i][j] = m1.data[i][j] + m2.data[i][j];
            }
        }
        let answer = Matrix::new(rows, cols, answer_data);
        Self {
            operands: vec![m1, m2],
            answer,
            op: Operation::Add,
        }
    }

    // 新しい引き算問題を生成
    pub fn new_sub(rows: usize, cols: usize, min_val: i32, max_val: i32, rng: &mut StdRng) -> Self {
        let m1 = Matrix::new_random(rows, cols, min_val, max_val, rng);
        let m2 = Matrix::new_random(rows, cols, min_val, max_val, rng);
        let mut answer_data = vec![vec![0; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                answer_data[i][j] = m1.data[i][j] - m2.data[i][j];
            }
        }
        let answer = Matrix::new(rows, cols, answer_data);
        Self {
            operands: vec![m1, m2],
            answer,
            op: Operation::Sub,
        }
    }

    // 新しい掛け算問題を生成
    pub fn new_mul(
        rows: usize,
        common_dim: usize,
        cols: usize,
        min_val: i32,
        max_val: i32,
        rng: &mut StdRng,
    ) -> Self {
        let m1 = Matrix::new_random(rows, common_dim, min_val, max_val, rng);
        let m2 = Matrix::new_random(common_dim, cols, min_val, max_val, rng);
        let mut answer_data = vec![vec![0; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                for k in 0..common_dim {
                    answer_data[i][j] += m1.data[i][k] * m2.data[k][j];
                }
            }
        }
        let answer = Matrix::new(rows, cols, answer_data);
        Self {
            operands: vec![m1, m2],
            answer,
            op: Operation::Mul,
        }
    }

    // Typst形式の文字列に変換
    pub fn to_typst(&self) -> String {
        let mut s = "".to_string();
        for (i, operand) in self.operands.iter().enumerate() {
            s.push_str(&operand.to_typst());
            if i < self.operands.len() - 1 {
                match self.op {
                    Operation::Add => s.push_str(" + "),
                    Operation::Sub => s.push_str(" - "),
                    Operation::Mul => s.push_str(" $times$ "),
                }
            }
        }
        s
    }

    // 解答をTypst形式の文字列に変換
    pub fn answer_to_typst(&self) -> String {
        self.answer.to_typst()
    }
}
