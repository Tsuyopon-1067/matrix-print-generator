use clap::Parser;
use matrix_print_generator::problem::Problem;
use matrix_print_generator::typst_generator;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of addition problems
    #[arg(long, default_value_t = 0)]
    add: u32,

    /// Number of subtraction problems
    #[arg(long, default_value_t = 0)]
    sub: u32,

    /// Number of multiplication problems
    #[arg(long, default_value_t = 0)]
    mul: u32,

    /// Number of rows for matrices
    #[arg(long, default_value_t = 3)]
    rows: usize,

    /// Number of columns for matrices
    #[arg(long, default_value_t = 3)]
    cols: usize,

    /// Common dimension for multiplication
    #[arg(long, default_value_t = 3)]
    common_dim: usize,

    /// Minimum value for matrix elements
    #[arg(long, default_value_t = 0)]
    min: i32,

    /// Maximum value for matrix elements
    #[arg(long, default_value_t = 10)]
    max: i32,

    /// Title of the generated document
    #[arg(short, long, default_value = "Matrix Practice")]
    title: String,

    /// Output file name (prefix)
    #[arg(short, long, default_value = "matrix_print")]
    output: String,

    /// Special flag to exit without doing anything
    #[arg(long, default_value_t = false)]
    m23: bool,
}

fn main() {
    let args = Args::parse();

    if args.m23 {
        println!("m23 flag is set. Exiting.");
        return;
    }

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

    if problems.is_empty() {
        println!(
            "No problems generated. Please specify the number of problems to generate using --add, --sub, or --mul."
        );
        return;
    }

    let result = typst_generator::generate_typst_file(&problems, &args.title, &args.output);
    match result {
        Ok(filename) => println!("Typst file generated: {}", filename),
        Err(e) => eprintln!("Error generating Typst file: {}", e),
    };
}
