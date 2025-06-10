pub fn get_input() -> (usize, usize, Vec<usize>) {
    let mut min_set_size = crate::constants::DEFAULT_MIN_SET_SIZE;
    let mut max_set_size = crate::constants::DEFAULT_MAX_SET_SIZE;

    let mut args = std::env::args().skip(1);
    let mut is_reading_properties = false;
    let mut property_indices: Vec<usize> = Vec::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--min" | "-m" => {
                is_reading_properties = false;

                if let Some(val) = args.next() {
                    min_set_size = val.parse().expect(
                        "Invalid minimum set size - should be a natural number (0, 1, 2, ...)",
                    );
                }
            }
            "--max" | "-M" => {
                is_reading_properties = false;

                if let Some(val) = args.next() {
                    max_set_size = val.parse().expect(
                        "Invalid maximum set size - should be a natural number (0, 1, 2, ...)",
                    );
                }
            }
            "--properties" | "-p" => {
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

    if max_set_size > crate::constants::MAX_SET_SIZE {
        panic!(
            "Invalid maximum set size - should be at most {}",
            crate::constants::MAX_SET_SIZE
        );
    }

    if min_set_size > max_set_size {
        panic!("Invalid set size range - minimum set size cannot be greater than maximum set size");
    }

    (min_set_size, max_set_size, property_indices)
}
