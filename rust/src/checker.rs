use crate::types::*;
use rayon::prelude::*;

pub fn start_checking(
    filename: &str,
    min_set_size: usize,
    max_set_size: usize,
    properties: PropertyVec,
) {
    for set_size in min_set_size..=max_set_size {
        let total_relations = 1 << (set_size * set_size);
        let start = std::time::Instant::now();
        let count = check_relations(set_size, total_relations, &properties);
        let elapsed = start.elapsed().as_secs_f64();

        crate::output::append_results_to_csv(
            filename,
            set_size,
            count,
            properties.len(),
            total_relations,
            elapsed,
        );
    }
}

fn check_relations(set_size: usize, total_relations: u64, properties: &PropertyVec) -> CountArray {
    (0..total_relations)
        .into_par_iter()
        .map(|i| {
            let mut relation = [0u8; crate::constants::MAX_SET_SIZE];
            let mut val = i;

            for j in 0..set_size {
                relation[j] = (val & ((1 << set_size) - 1)) as u8;
                val >>= set_size;
            }

            let mut local = [0u64; crate::constants::NUMBER_OF_PROPERTIES];

            for (index, &property) in properties.iter().enumerate() {
                local[index] =
                    crate::properties::PROPERTY_FUNCTIONS[property](relation, set_size) as u64;
            }

            local
        })
        .reduce(
            || [0u64; crate::constants::NUMBER_OF_PROPERTIES],
            |mut a, b| {
                for i in 0..properties.len() {
                    a[i] += b[i];
                }

                a
            },
        )
}
