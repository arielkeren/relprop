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
    count: Vec<u64>,
    total_relations: u64,
    elapsed: f64,
) {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(filename)
        .expect("Failed to open file for appending");

    let mut line = format!("{},{},{}", set_size, total_relations, elapsed);

    for &property_count in count.iter() {
        let pct = (property_count as f64 / total_relations as f64) * 100.0;
        line.push_str(&format!(",{},{}", property_count, pct));
    }

    line.push('\n');

    file.write_all(line.as_bytes())
        .expect("Failed to write results to CSV");
}

pub fn print_results(
    set_size: usize,
    count: &Vec<u64>,
    total_relations: u64,
    elapsed: f64,
    properties: &Vec<usize>,
) {
    println!("-------------------");
    println!("Set size: {}", set_size);
    println!("Total relations: {}", total_relations);
    println!("Time to check: {} seconds", elapsed);

    for (&property, &property_count) in properties.iter().zip(count.iter()) {
        println!("{}", crate::constants::CAPITALIZED_PROPERTY_NAMES[property]);
        println!("Total: {}", property_count);
        println!(
            "Percentage: {}%\n",
            (property_count as f64 / total_relations as f64) * 100.0
        );
    }
}
