use clap::Parser;
use matrix_print_generator::{
    pdf_generator,
    problem_generator::{Args, generate_problems, generate_problems_rutine},
    seed_rng, typst_generator,
};
use seed_rng::SeedRng;

fn main() {
    let args = Args::parse();

    let mut seed_rng = SeedRng::new(args.seed);
    let problems = if args.m23 {
        generate_problems_rutine(&args, &mut seed_rng.rng)
    } else {
        generate_problems(&args, &mut seed_rng.rng)
    };

    if problems.is_empty() {
        println!(
            "No problems generated. Please specify the number of problems to generate using --add, --sub, or --mul."
        );
        return;
    }

    let result =
        typst_generator::generate_typst_file(&problems, &args.title, &args.output, seed_rng.seed);
    match result {
        Ok(filename) => {
            println!("Typst file generated: {}", filename);
            pdf_generator::generate_pdf(&filename);
        }
        Err(e) => eprintln!("Error generating Typst file: {}", e),
    };
}
