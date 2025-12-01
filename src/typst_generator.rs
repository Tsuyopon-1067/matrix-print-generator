use crate::problem::Problem;
use chrono::Local;
use std::io::Write;

pub fn generate_typst_file(
    problems: &[Problem],
    title: &str,
    output_prefix: &str,
    seed: u64,
) -> std::io::Result<String> {
    let mut file_content = String::new();
    file_content.push_str(&format!(
        "#set page(header: align(right, text(size: 10pt, [seed={}])))\n\n",
        seed,
    ));

    file_content.push_str(&format!("= {}\n\n", title));
    file_content.push_str("== 問題\n\n");

    for (i, p) in problems.iter().enumerate() {
        file_content.push_str(&format!("{}. {}\n\n", i + 1, p.to_typst()));
    }

    file_content.push_str("\n\n#pagebreak()\n\n");
    file_content.push_str("\n\n== 解答\n\n");

    for (i, p) in problems.iter().enumerate() {
        file_content.push_str(&format!("{}. {}\n\n", i + 1, p.answer_to_typst()));
    }

    let dt = Local::now();
    let filename = format!("{}_{}.typ", output_prefix, dt.format("%Y%m%d_%H%M%S"));

    let mut file = std::fs::File::create(&filename)?;
    file.write_all(file_content.as_bytes())?;

    println!("Generated {}", filename);
    Ok(filename)
}
