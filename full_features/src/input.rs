use crate::types::*;

pub fn get_input() -> (usize, usize, PropertyVec) {
    let mut min_set_size: Option<usize> = None;
    let mut max_set_size: Option<usize> = None;

    let mut is_reading_properties = false;
    let mut has_read_properties = false;

    let mut args = std::env::args().skip(1);
    let mut property_indices: PropertyVec = Vec::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--min" | "-m" => {
                if min_set_size.is_some() {
                    panic!("Minimum set size specified multiple times");
                }

                is_reading_properties = false;

                if let Some(val) = args.next() {
                    min_set_size = Some(val.parse().expect(
                        "Invalid minimum set size - should be a natural number (0, 1, 2, ...)",
                    ));
                }
            }
            "--max" | "-M" => {
                if max_set_size.is_some() {
                    panic!("Maximum set size specified multiple times");
                }

                is_reading_properties = false;

                if let Some(val) = args.next() {
                    max_set_size = Some(val.parse().expect(
                        "Invalid maximum set size - should be a natural number (0, 1, 2, ...)",
                    ));
                }
            }
            "--properties" | "-p" => {
                if has_read_properties {
                    panic!("Properties specified multiple times");
                }

                has_read_properties = true;
                is_reading_properties = true;
            }
            _ => {
                if !is_reading_properties {
                    panic!("Unknown argument: {}.", arg);
                }

                match crate::constants::PROPERTY_NAMES
                    .iter()
                    .position(|&name| name == arg.to_lowercase())
                {
                    Some(index) => {
                        property_indices.push(index);
                    }
                    None => {
                        panic!(
                            "Invalid property: {} - Valid properties are: {:?}",
                            arg,
                            crate::constants::PROPERTY_NAMES
                        );
                    }
                }
            }
        }
    }

    if has_read_properties && property_indices.is_empty() {
        panic!("Flag is present, but no properties specified");
    }

    let min_size = min_set_size.unwrap_or(crate::constants::DEFAULT_MIN_SET_SIZE);
    let max_size = max_set_size.unwrap_or(crate::constants::DEFAULT_MAX_SET_SIZE);

    validate_set_size(min_size, max_size);

    if property_indices.is_empty() {
        return (
            min_size,
            max_size,
            (0..crate::constants::PROPERTY_NAMES.len()).collect(),
        );
    }

    property_indices.sort_unstable();

    (min_size, max_size, property_indices)
}

fn validate_set_size(min_set_size: usize, max_set_size: usize) {
    if max_set_size > crate::constants::MAX_SET_SIZE {
        panic!(
            "Invalid maximum set size - should be at most {}",
            crate::constants::MAX_SET_SIZE
        );
    }

    if min_set_size > max_set_size {
        panic!("Invalid set size range - minimum set size cannot be greater than maximum set size");
    }
}
