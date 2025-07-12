use clap::Parser;
use matrix_print_generator::problem_generator::{
    Args, generate_problems, generate_problems_rutine,
};
use matrix_print_generator::typst_generator;

fn main() {
    let args = Args::parse();

    let problems = if args.m23 {
        generate_problems_rutine(&args)
    } else {
        generate_problems(&args)
    };

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
