pub const PROPERTY_FUNCTIONS: [fn([u8; crate::constants::MAX_SET_SIZE], u8) -> bool;
    crate::constants::NUMBER_OF_PROPERTIES] = [
    antisymmetry,
    antitransitivity,
    asymmetry,
    coreflexivity,
    density,
    irreflexivity,
    left_euclideanness,
    left_quasi_reflexivity,
    quasi_reflexivity,
    reflexivity,
    right_euclideanness,
    right_quasi_reflexivity,
    strict_density,
    symmetry,
    totality,
    transitivity,
    trichotomy,
];

fn get(relation: [u8; crate::constants::MAX_SET_SIZE], row: u8, col: u8) -> bool {
    (relation[row as usize] >> col) & 1 == 1
}

fn antisymmetry(relation: [u8; crate::constants::MAX_SET_SIZE], set_size: u8) -> bool {
    for i in 0..set_size {
        for j in i + 1..set_size {
            if get(relation, i, j) && get(relation, j, i) {
                return false;
            }
        }
    }

    true
}

fn antitransitivity(relation: [u8; crate::constants::MAX_SET_SIZE], set_size: u8) -> bool {
    for i in 0..set_size {
        if relation[i as usize] == 0 {
            continue;
        }

        for j in 0..set_size {
            if get(relation, i, j) && (relation[i as usize] & relation[j as usize] != 0) {
                return false;
            }
        }
    }

    true
}

fn asymmetry(relation: [u8; crate::constants::MAX_SET_SIZE], set_size: u8) -> bool {
    for i in 0..set_size {
        if get(relation, i, i) {
            return false;
        }

        for j in i + 1..set_size {
            if get(relation, i, j) && get(relation, j, i) {
                return false;
            }
        }
    }

    true
}

fn coreflexivity(relation: [u8; crate::constants::MAX_SET_SIZE], set_size: u8) -> bool {
    for i in 0..set_size {
        if relation[i as usize] & !(1 << i) != 0 {
            return false;
        }
    }

    true
}

fn density(relation: [u8; crate::constants::MAX_SET_SIZE], set_size: u8) -> bool {
    for i in 0..set_size {
        for j in 0..set_size {
            if !get(relation, i, j) {
                continue;
            }

            let mut found = false;

            for k in 0..set_size {
                if get(relation, i, k) && get(relation, k, j) {
                    found = true;
                    break;
                }
            }

            if !found {
                return false;
            }
        }
    }

    true
}

fn irreflexivity(relation: [u8; crate::constants::MAX_SET_SIZE], set_size: u8) -> bool {
    for i in 0..set_size {
        if get(relation, i, i) {
            return false;
        }
    }

    true
}

fn left_euclideanness(relation: [u8; crate::constants::MAX_SET_SIZE], set_size: u8) -> bool {
    for i in 0..set_size {
        for j in 0..set_size {
            for k in 0..set_size {
                if get(relation, j, i) && get(relation, k, j) && !get(relation, k, i) {
                    return false;
                }
            }
        }
    }

    true
}

fn left_quasi_reflexivity(relation: [u8; crate::constants::MAX_SET_SIZE], set_size: u8) -> bool {
    for i in 0..set_size {
        for j in 0..set_size {
            if i != j && get(relation, i, j) && !get(relation, i, i) {
                return false;
            }
        }
    }

    true
}

fn quasi_reflexivity(relation: [u8; crate::constants::MAX_SET_SIZE], set_size: u8) -> bool {
    for i in 0..set_size {
        for j in 0..set_size {
            if i != j && get(relation, i, j) && !(get(relation, i, i) && get(relation, j, j)) {
                return false;
            }
        }
    }

    true
}

fn reflexivity(relation: [u8; crate::constants::MAX_SET_SIZE], set_size: u8) -> bool {
    for i in 0..set_size {
        if !get(relation, i, i) {
            return false;
        }
    }

    true
}

fn right_euclideanness(relation: [u8; crate::constants::MAX_SET_SIZE], set_size: u8) -> bool {
    for i in 0..set_size {
        for j in 0..set_size {
            for k in 0..set_size {
                if get(relation, i, j) && get(relation, i, k) && !get(relation, j, k) {
                    return false;
                }
            }
        }
    }

    true
}

fn right_quasi_reflexivity(relation: [u8; crate::constants::MAX_SET_SIZE], set_size: u8) -> bool {
    for i in 0..set_size {
        for j in 0..set_size {
            if i != j && get(relation, i, j) && !get(relation, j, j) {
                return false;
            }
        }
    }

    true
}

fn strict_density(relation: [u8; crate::constants::MAX_SET_SIZE], set_size: u8) -> bool {
    for i in 0..set_size {
        for j in 0..set_size {
            if i == j || !get(relation, i, j) {
                continue;
            }

            let mut found = false;

            for k in 0..set_size {
                if k == i || k == j {
                    continue;
                }

                if get(relation, i, k) && get(relation, k, j) {
                    found = true;
                    break;
                }
            }

            if !found {
                return false;
            }
        }
    }

    true
}

fn symmetry(relation: [u8; crate::constants::MAX_SET_SIZE], set_size: u8) -> bool {
    for i in 0..set_size {
        for j in i + 1..set_size {
            if get(relation, i, j) != get(relation, j, i) {
                return false;
            }
        }
    }

    true
}

fn totality(relation: [u8; crate::constants::MAX_SET_SIZE], set_size: u8) -> bool {
    for i in 0..set_size {
        if !get(relation, i, i) {
            return false;
        }

        for j in i + 1..set_size {
            if !(get(relation, i, j) || get(relation, j, i)) {
                return false;
            }
        }
    }

    true
}

fn transitivity(relation: [u8; crate::constants::MAX_SET_SIZE], set_size: u8) -> bool {
    for i in 0..set_size {
        if relation[i as usize] == 0 {
            continue;
        }

        for j in 0..set_size {
            if get(relation, i, j) && (relation[j as usize] & !relation[i as usize] != 0) {
                return false;
            }
        }
    }

    true
}

fn trichotomy(relation: [u8; crate::constants::MAX_SET_SIZE], set_size: u8) -> bool {
    for i in 0..set_size {
        for j in i + 1..set_size {
            if !(get(relation, i, j) || get(relation, j, i)) {
                return false;
            }
        }
    }

    true
}
