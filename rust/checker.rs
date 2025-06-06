pub fn start_checking(filename: &str, min_set_size: u8, max_set_size: u8) {
    for set_size in min_set_size..=max_set_size {
        let total_relations = 1 << (set_size * set_size);
        let start = std::time::Instant::now();
        let count = check_relations(set_size, total_relations);
        let elapsed = start.elapsed().as_secs_f64();

        crate::summary::print_results(set_size, count, total_relations, elapsed);
        crate::summary::append_results_to_csv(filename, set_size, count, total_relations, elapsed);
    }
}

pub fn check_relations(
    set_size: u8,
    total_relations: u64,
) -> [u64; crate::constants::NUMBER_OF_PROPERTIES] {
    let mut count: [u64; crate::constants::NUMBER_OF_PROPERTIES] =
        [0; crate::constants::NUMBER_OF_PROPERTIES];
    let mut relation: [u8; crate::constants::MAX_SET_SIZE] = [0; crate::constants::MAX_SET_SIZE];

    for i in 0..total_relations {
        let mut val = i;

        for j in 0..set_size {
            relation[j as usize] = (val & ((1 << set_size) - 1)) as u8;
            val >>= set_size;
        }

        for j in 0..crate::constants::NUMBER_OF_PROPERTIES {
            count[j] += crate::properties::PROPERTY_FUNCTIONS[j](relation, set_size) as u64;
        }
    }

    count
}
