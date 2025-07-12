use crate::problem::Problem;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Number of addition problems
    #[arg(long, default_value_t = 0)]
    pub add: u32,

    /// Number of subtraction problems
    #[arg(long, default_value_t = 0)]
    pub sub: u32,

    /// Number of multiplication problems
    #[arg(long, default_value_t = 0)]
    pub mul: u32,

    /// Number of rows for matrices
    #[arg(long, default_value_t = 3)]
    pub rows: usize,

    /// Number of columns for matrices
    #[arg(long, default_value_t = 3)]
    pub cols: usize,

    /// Common dimension for multiplication
    #[arg(long, default_value_t = 3)]
    pub common_dim: usize,

    /// Minimum value for matrix elements
    #[arg(long, default_value_t = 0)]
    pub min: i32,

    /// Maximum value for matrix elements
    #[arg(long, default_value_t = 10)]
    pub max: i32,

    /// Title of the generated document
    #[arg(short, long, default_value = "Matrix Practice")]
    pub title: String,

    /// Output file name (prefix)
    #[arg(short, long, default_value = "matrix_print")]
    pub output: String,

    /// special flag to exit without doing anything
    #[arg(long, default_value_t = false)]
    pub m23: bool,

    /// special flag to exit without doing anything
    #[arg(long, default_value_t = false)]
    pub pm: bool,
}

pub fn generate_problems(args: &Args) -> Vec<Problem> {
    let mut problems = Vec::new();

    // Generate addition problems
    for _ in 0..args.add {
        problems.push(Problem::new_add(args.rows, args.cols, args.min, args.max));
    }

    // Generate subtraction problems
    for _ in 0..args.sub {
        problems.push(Problem::new_sub(args.rows, args.cols, args.min, args.max));
    }

    // Generate multiplication problems
    for _ in 0..args.mul {
        problems.push(Problem::new_mul(
            args.rows,
            args.common_dim,
            args.cols,
            args.min,
            args.max,
        ));
    }
    problems
}

pub fn generate_problems_rutine(args: &Args) -> Vec<Problem> {
    let mut problems = Vec::new();
    let add_min = if args.pm { -99999 } else { 0 };
    let mul_min = if args.pm { -99 } else { 0 };

    // Generate addition problems
    for _ in 0..3 {
        problems.push(Problem::new_add(3, 3, add_min, 99999));
    }
    // Generate multiplication problems
    for _ in 0..3 {
        problems.push(Problem::new_mul(2, 2, 2, mul_min, 99));
    }
    for _ in 0..2 {
        problems.push(Problem::new_mul(3, 3, 3, mul_min, 99));
    }
    for _ in 0..2 {
        problems.push(Problem::new_mul(4, 4, 4, mul_min, 99));
    }
    problems
}
