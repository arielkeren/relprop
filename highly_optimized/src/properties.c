#include "properties.h"

#include <stdbool.h>
#include <stdint.h>

#include "constants.h"

bool (*const PROPERTY_FUNCTIONS[NUMBER_OF_PROPERTIES])(uint8_t[], uint8_t) = {
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
    trichotomy};

bool antisymmetry(uint8_t relation[MAX_SET_SIZE], uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++) {
        uint8_t row = relation[i];

        for (uint8_t j = i + 1; j < set_size; j++)
            if (GET_ROW(row, j) && GET(relation, j, i)) return false;
    }

    return true;
}

bool antitransitivity(uint8_t relation[MAX_SET_SIZE], uint8_t set_size) {
    for (int i = 0; i < set_size; i++) {
        uint8_t row = relation[i];
        if (!row) continue;

        for (int j = 0; j < set_size; j++)
            if ((GET_ROW(row, j)) && (row & relation[j])) return false;
    }

    return true;
}

bool asymmetry(uint8_t relation[MAX_SET_SIZE], uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++) {
        if (GET(relation, i, i)) return false;
    }

    for (uint8_t i = 0; i < set_size; i++) {
        uint8_t row = relation[i];

        for (uint8_t j = i + 1; j < set_size; j++)
            if (GET_ROW(row, j) && GET(relation, j, i)) return false;
    }

    return true;
}

bool coreflexivity(uint8_t relation[MAX_SET_SIZE], uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++)
        if (relation[i] & ~(1 << i)) return false;

    return true;
}

bool density(uint8_t relation[MAX_SET_SIZE], uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++) {
        uint8_t row = relation[i];
        if (!row) continue;

        for (uint8_t j = 0; j < set_size; j++) {
            if (!GET_ROW(row, j)) continue;

            bool found = false;

            for (uint8_t k = 0; k < set_size; k++) {
                if (GET_ROW(row, k) && GET(relation, k, j)) {
                    found = true;
                    break;
                }
            }

            if (!found) return false;
        }
    }

    return true;
}

bool irreflexivity(uint8_t relation[MAX_SET_SIZE], uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++)
        if (GET(relation, i, i)) return false;

    return true;
}

bool left_euclideanness(uint8_t relation[MAX_SET_SIZE], uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++)
        for (uint8_t j = 0; j < set_size; j++) {
            uint8_t row = relation[j];

            for (uint8_t k = 0; k < set_size; k++)
                if (GET_ROW(row, i) && GET(relation, k, i) && !GET_ROW(row, k))
                    return false;
        }

    return true;
}

bool left_quasi_reflexivity(uint8_t relation[MAX_SET_SIZE], uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++) {
        uint8_t row = relation[i];

        for (uint8_t j = 0; j < set_size; j++)
            if (i != j && GET_ROW(row, j) && !GET_ROW(row, i)) return false;
    }

    return true;
}

bool quasi_reflexivity(uint8_t relation[MAX_SET_SIZE], uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++) {
        uint8_t row = relation[i];

        for (uint8_t j = 0; j < set_size; j++)
            if (i != j && GET_ROW(row, j) &&
                !(GET_ROW(row, i) && GET(relation, j, j)))
                return false;
    }

    return true;
}

bool reflexivity(uint8_t relation[MAX_SET_SIZE], uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++)
        if (!GET(relation, i, i)) return false;

    return true;
}

bool right_euclideanness(uint8_t relation[MAX_SET_SIZE], uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++) {
        uint8_t row_i = relation[i];

        for (uint8_t j = 0; j < set_size; j++) {
            uint8_t row_j = relation[j];

            for (uint8_t k = 0; k < set_size; k++)
                if (GET_ROW(row_i, j) && GET_ROW(row_i, k) &&
                    !GET_ROW(row_j, k))
                    return false;
        }
    }

    return true;
}

bool right_quasi_reflexivity(uint8_t relation[MAX_SET_SIZE], uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++) {
        uint8_t row = relation[i];

        for (uint8_t j = 0; j < set_size; j++)
            if (i != j && GET_ROW(row, j) && !GET(relation, j, j)) return false;
    }

    return true;
}

bool strict_density(uint8_t relation[MAX_SET_SIZE], uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++) {
        uint8_t row = relation[i];
        if (!row) continue;

        for (uint8_t j = 0; j < set_size; j++) {
            if (i == j || !GET_ROW(row, j)) continue;

            bool found = false;

            for (uint8_t k = 0; k < set_size; k++) {
                if (k == i || k == j) continue;
                if (GET_ROW(row, k) && GET(relation, k, j)) {
                    found = true;
                    break;
                }
            }

            if (!found) return false;
        }
    }

    return true;
}

bool symmetry(uint8_t relation[MAX_SET_SIZE], uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++) {
        uint8_t row = relation[i];

        for (uint8_t j = i + 1; j < set_size; j++)
            if (GET_ROW(row, j) != GET(relation, j, i)) return false;
    }

    return true;
}

bool totality(uint8_t relation[MAX_SET_SIZE], uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++) {
        if (!GET(relation, i, i)) return false;
    }

    for (uint8_t i = 0; i < set_size; i++) {
        uint8_t row = relation[i];

        for (uint8_t j = i + 1; j < set_size; j++)
            if (!(GET_ROW(row, j) | GET(relation, j, i))) return false;
    }

    return true;
}

bool transitivity(uint8_t relation[MAX_SET_SIZE], uint8_t set_size) {
    for (int i = 0; i < set_size; i++) {
        uint8_t row = relation[i];
        if (!row) continue;

        for (int j = 0; j < set_size; j++)
            if ((GET_ROW(row, j)) && (relation[j] & ~row)) return false;
    }

    return true;
}

bool trichotomy(uint8_t relation[MAX_SET_SIZE], uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++) {
        uint8_t row = relation[i];

        for (uint8_t j = i + 1; j < set_size; j++)
            if (!(GET_ROW(row, j) | GET(relation, j, i))) return false;
    }

    return true;
}
