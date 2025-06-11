use crate::types::*;

pub const PROPERTY_FUNCTIONS: [PropertyFn; crate::constants::NUMBER_OF_PROPERTIES] = [
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

macro_rules! get {
    ($relation:expr, $row:expr, $col:expr) => {
        ($relation[$row] >> $col) & 1 == 1
    };
}

fn antisymmetry(relation: Relation, set_size: usize) -> bool {
    for i in 0..set_size {
        for j in i + 1..set_size {
            if get!(relation, i, j) && get!(relation, j, i) {
                return false;
            }
        }
    }

    true
}

fn antitransitivity(relation: Relation, set_size: usize) -> bool {
    for i in 0..set_size {
        if relation[i] == 0 {
            continue;
        }

        for j in 0..set_size {
            if get!(relation, i, j) && (relation[i] & relation[j] != 0) {
                return false;
            }
        }
    }

    true
}

fn asymmetry(relation: Relation, set_size: usize) -> bool {
    for i in 0..set_size {
        if get!(relation, i, i) {
            return false;
        }

        for j in i + 1..set_size {
            if get!(relation, i, j) && get!(relation, j, i) {
                return false;
            }
        }
    }

    true
}

fn coreflexivity(relation: Relation, set_size: usize) -> bool {
    for i in 0..set_size {
        if relation[i] & !(1 << i) != 0 {
            return false;
        }
    }

    true
}

fn density(relation: Relation, set_size: usize) -> bool {
    for i in 0..set_size {
        for j in 0..set_size {
            if !get!(relation, i, j) {
                continue;
            }

            let mut found = false;

            for k in 0..set_size {
                if get!(relation, i, k) && get!(relation, k, j) {
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

fn irreflexivity(relation: Relation, set_size: usize) -> bool {
    for i in 0..set_size {
        if get!(relation, i, i) {
            return false;
        }
    }

    true
}

fn left_euclideanness(relation: Relation, set_size: usize) -> bool {
    for i in 0..set_size {
        for j in 0..set_size {
            for k in 0..set_size {
                if get!(relation, j, i) && get!(relation, k, j) && !get!(relation, k, i) {
                    return false;
                }
            }
        }
    }

    true
}

fn left_quasi_reflexivity(relation: Relation, set_size: usize) -> bool {
    for i in 0..set_size {
        for j in 0..set_size {
            if i != j && get!(relation, i, j) && !get!(relation, i, i) {
                return false;
            }
        }
    }

    true
}

fn quasi_reflexivity(relation: Relation, set_size: usize) -> bool {
    for i in 0..set_size {
        for j in 0..set_size {
            if i != j && get!(relation, i, j) && !(get!(relation, i, i) && get!(relation, j, j)) {
                return false;
            }
        }
    }

    true
}

fn reflexivity(relation: Relation, set_size: usize) -> bool {
    for i in 0..set_size {
        if !get!(relation, i, i) {
            return false;
        }
    }

    true
}

fn right_euclideanness(relation: Relation, set_size: usize) -> bool {
    for i in 0..set_size {
        for j in 0..set_size {
            for k in 0..set_size {
                if get!(relation, i, j) && get!(relation, i, k) && !get!(relation, j, k) {
                    return false;
                }
            }
        }
    }

    true
}

fn right_quasi_reflexivity(relation: Relation, set_size: usize) -> bool {
    for i in 0..set_size {
        for j in 0..set_size {
            if i != j && get!(relation, i, j) && !get!(relation, j, j) {
                return false;
            }
        }
    }

    true
}

fn strict_density(relation: Relation, set_size: usize) -> bool {
    for i in 0..set_size {
        for j in 0..set_size {
            if i == j || !get!(relation, i, j) {
                continue;
            }

            let mut found = false;

            for k in 0..set_size {
                if k == i || k == j {
                    continue;
                }

                if get!(relation, i, k) && get!(relation, k, j) {
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

fn symmetry(relation: Relation, set_size: usize) -> bool {
    for i in 0..set_size {
        for j in i + 1..set_size {
            if get!(relation, i, j) != get!(relation, j, i) {
                return false;
            }
        }
    }

    true
}

fn totality(relation: Relation, set_size: usize) -> bool {
    for i in 0..set_size {
        if !get!(relation, i, i) {
            return false;
        }

        for j in i + 1..set_size {
            if !(get!(relation, i, j) || get!(relation, j, i)) {
                return false;
            }
        }
    }

    true
}

fn transitivity(relation: Relation, set_size: usize) -> bool {
    for i in 0..set_size {
        if relation[i] == 0 {
            continue;
        }

        for j in 0..set_size {
            if get!(relation, i, j) && (relation[j] & !relation[i] != 0) {
                return false;
            }
        }
    }

    true
}

fn trichotomy(relation: Relation, set_size: usize) -> bool {
    for i in 0..set_size {
        for j in i + 1..set_size {
            if !(get!(relation, i, j) || get!(relation, j, i)) {
                return false;
            }
        }
    }

    true
}
