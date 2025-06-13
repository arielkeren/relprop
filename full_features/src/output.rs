use crate::types::*;
use std::io::Write;

pub fn write_csv_header(filename: &str, properties: &PropertyVec) {
    let mut file = std::fs::File::create(filename).expect("Failed to open file for writing header");

    let mut header = "Set,Total,Time".to_string();
    add_property_headers(&mut header, properties);
    header.push('\n');

    file.write_all(header.as_bytes())
        .expect("Failed to write header to CSV");
}

pub fn append_results_to_csv(
    filename: &str,
    set_size: usize,
    count: CountArray,
    number_of_properties: usize,
    total_relations: u64,
    elapsed: f64,
) {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(filename)
        .expect("Failed to open file for appending");

    let mut line = format!("{},{},{}", set_size, total_relations, elapsed);
    add_property_results(&mut line, count, number_of_properties, total_relations);
    line.push('\n');

    file.write_all(line.as_bytes())
        .expect("Failed to write results to CSV");
}

fn add_property_headers(header: &mut String, properties: &PropertyVec) {
    for &property in properties.iter() {
        header.push_str(&format!(
            ",{}_Count,{}_Pct",
            crate::constants::CAPITALIZED_PROPERTY_NAMES[property],
            crate::constants::CAPITALIZED_PROPERTY_NAMES[property]
        ));
    }
}

fn add_property_results(
    line: &mut String,
    count: CountArray,
    number_of_properties: usize,
    total_relations: u64,
) {
    for property in 0..number_of_properties {
        if total_relations == 0 {
            line.push_str(",0,0");
            continue;
        }

        let pct = (count[property] as f64 / total_relations as f64) * 100.0;
        line.push_str(&format!(",{},{}", count[property], pct));
    }
}
