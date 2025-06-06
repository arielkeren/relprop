use std::io::Write;

pub fn write_csv_header(filename: &str) {
    let mut file = std::fs::File::create(filename).expect("Failed to open file for writing header");

    let mut header = "Set,Total,Time".to_string();
    for property_name in crate::constants::PROPERTY_NAMES.iter() {
        header.push_str(&format!(",{}_Count,{}_Pct", property_name, property_name));
    }
    header.push('\n');

    file.write_all(header.as_bytes())
        .expect("Failed to write header to CSV");
}

pub fn append_results_to_csv(
    filename: &str,
    set_size: u8,
    count: [u64; crate::constants::NUMBER_OF_PROPERTIES],
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
    set_size: u8,
    count: [u64; crate::constants::NUMBER_OF_PROPERTIES],
    total_relations: u64,
    elapsed: f64,
) {
    println!("-------------------");
    println!("Set size: {}", set_size);
    println!("Total relations: {}", total_relations);
    println!("Time to check: {} seconds", elapsed);

    for (property_name, &property_count) in
        crate::constants::PROPERTY_NAMES.iter().zip(count.iter())
    {
        println!("{}", property_name);
        println!("Total: {}", property_count);
        println!(
            "Percentage: {}%\n",
            (property_count as f64 / total_relations as f64) * 100.0
        );
    }
}
