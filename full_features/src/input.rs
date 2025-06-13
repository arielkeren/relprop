use crate::types::*;

pub fn get_input() -> (usize, usize, PropertyVec) {
    let (min_set_size, max_set_size, mut properties) = read_args();

    let min_size = min_set_size.unwrap_or(crate::constants::DEFAULT_MIN_SET_SIZE);
    let max_size = max_set_size.unwrap_or(crate::constants::DEFAULT_MAX_SET_SIZE);

    validate_set_size(min_size, max_size);

    if properties.is_empty() {
        return (
            min_size,
            max_size,
            (0..crate::constants::PROPERTY_NAMES.len()).collect(),
        );
    }

    properties.sort_unstable();

    (min_size, max_size, properties)
}

fn read_args() -> (Option<usize>, Option<usize>, Vec<usize>) {
    let mut min_set_size: Option<usize> = None;
    let mut max_set_size: Option<usize> = None;

    let mut is_reading_properties = false;
    let mut has_read_properties = false;

    let mut args = std::env::args().skip(1);
    let mut properties: PropertyVec = Vec::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--min" | "-m" => {
                if min_set_size.is_some() {
                    arg_error("Minimum set size specified multiple times");
                }

                is_reading_properties = false;

                if let Some(val) = args.next() {
                    min_set_size = Some(val.parse().unwrap_or_else(|_| {
                        arg_error(
                            "Invalid minimum set size - should be a natural number (0, 1, 2, ...)",
                        )
                    }));
                }
            }
            "--max" | "-M" => {
                if max_set_size.is_some() {
                    arg_error("Maximum set size specified multiple times");
                }

                is_reading_properties = false;

                if let Some(val) = args.next() {
                    max_set_size = Some(val.parse().unwrap_or_else(|_| {
                        arg_error(
                            "Invalid maximum set size - should be a natural number (0, 1, 2, ...)",
                        )
                    }));
                }
            }
            "--properties" | "-p" => {
                if has_read_properties {
                    arg_error("Properties specified multiple times");
                }

                has_read_properties = true;
                is_reading_properties = true;
            }
            _ => {
                if !is_reading_properties {
                    arg_error(format!("Unknown argument: {}.", arg).as_str());
                }

                match crate::constants::PROPERTY_NAMES
                    .iter()
                    .position(|&name| name == arg.to_lowercase())
                {
                    Some(index) => {
                        properties.push(index);
                    }
                    None => {
                        arg_error(format!("Invalid property: {}", arg).as_str());
                    }
                }
            }
        }
    }

    if has_read_properties && properties.is_empty() {
        arg_error("Flag is present, but no properties specified");
    }

    return (min_set_size, max_set_size, properties);
}

fn validate_set_size(min_set_size: usize, max_set_size: usize) {
    if max_set_size > crate::constants::MAX_SET_SIZE {
        arg_error(
            format!(
                "Invalid maximum set size - should be at most {}",
                crate::constants::MAX_SET_SIZE
            )
            .as_str(),
        );
    }

    if min_set_size > max_set_size {
        arg_error(
            "Invalid set size range - minimum set size cannot be greater than maximum set size",
        );
    }
}

fn arg_error(error: &str) -> ! {
    let program_name = std::env::args().next().unwrap_or("program".to_string());

    eprintln!("ERROR: {}", error);
    eprintln!(
        "Usage: {} [--min <min_size>] [--max <max_size>] [--properties <property1> <property2> ...]",
        program_name
    );
    eprintln!(
        "Short form: {} [-m <min_size>] [-M <max_size>] [-p <property1> <property2> ...]",
        program_name
    );
    eprintln!(
        "Minimum set size defaults to {}, maximum set size defaults to {}, and property list defaults to all properties",
        crate::constants::DEFAULT_MIN_SET_SIZE,
        crate::constants::DEFAULT_MAX_SET_SIZE,
    );
    eprintln!(
        "Set sizes have to be between 0 and {}",
        crate::constants::MAX_SET_SIZE
    );
    eprintln!(
        "Valid properties are: {:?}",
        crate::constants::PROPERTY_NAMES
    );
    std::process::exit(1);
}
