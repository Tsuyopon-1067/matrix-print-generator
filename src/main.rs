use matrix_print_generator::problem::Problem;
use matrix_print_generator::typst_generator;

fn main() {
    let mut problems = Vec::new();
    for _ in 0..500 {
        problems.push(Problem::new_add(3, 3, 0, 10));
    }

    let result = typst_generator::generate_typst_file(&problems);
    match result {
        Ok(filename) => println!("Typst file generated: {}", filename),
        Err(e) => eprintln!("Error generating Typst file: {}", e),
    };
}
