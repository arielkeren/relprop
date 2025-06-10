use std::io::Write;

pub fn write_csv_header(filename: &str, properties: &Vec<usize>) {
    let mut file = std::fs::File::create(filename).expect("Failed to open file for writing header");

    let mut header = "Set,Total,Time".to_string();
    for &property in properties.iter() {
        header.push_str(&format!(
            ",{}_Count,{}_Pct",
            crate::constants::CAPITALIZED_PROPERTY_NAMES[property],
            crate::constants::CAPITALIZED_PROPERTY_NAMES[property]
        ));
    }
    header.push('\n');

    file.write_all(header.as_bytes())
        .expect("Failed to write header to CSV");
}

pub fn append_results_to_csv(
    filename: &str,
    set_size: usize,
    count: [u64; crate::constants::NUMBER_OF_PROPERTIES],
    number_of_properties: usize,
    total_relations: u64,
    elapsed: f64,
) {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(filename)
        .expect("Failed to open file for appending");

    let mut line = format!("{},{},{}", set_size, total_relations, elapsed);

    for index in 0..number_of_properties {
        let pct = (count[index] as f64 / total_relations as f64) * 100.0;
        line.push_str(&format!(",{},{}", count[index], pct));
    }

    line.push('\n');

    file.write_all(line.as_bytes())
        .expect("Failed to write results to CSV");
}
