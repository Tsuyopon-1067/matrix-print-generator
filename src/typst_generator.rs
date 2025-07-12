use crate::problem::Problem;
use chrono::Local;
use std::io::Write;

pub fn generate_typst_file(problems: &[Problem]) -> std::io::Result<String> {
    let mut file_content = "".to_string();
    file_content.push_str("== 問題\n\n");

    file_content.push_str("#columns(2, gutter: 1cm)[\n");
    for (i, p) in problems.iter().enumerate() {
        file_content.push_str(&format!("{}. {}\n\n", i + 1, p.to_typst()));
    }
    file_content.push_str("]\n");

    file_content.push_str("\n\n#pagebreak()\n\n");
    file_content.push_str("\n\n== 解答\n\n");

    file_content.push_str("#columns(2, gutter: 1cm)[\n");
    for (i, p) in problems.iter().enumerate() {
        file_content.push_str(&format!("{}. {}\n\n", i + 1, p.answer_to_typst()));
    }
    file_content.push_str("]\n");

    let dt = Local::now();
    let filename = format!("tmp_{}.typ", dt.format("%Y%m%d_%H%M%S"));

    let mut file = std::fs::File::create(&filename)?;
    file.write_all(file_content.as_bytes())?;

    println!("Generated {}", filename);
    Ok(filename)
}
