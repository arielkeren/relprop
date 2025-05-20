#ifndef PROPERTIES_H
#define PROPERTIES_H

#include <stdbool.h>
#include <stdint.h>

#define NUMBER_OF_PROPERTIES 9

#define GET(relation, set_size, row, col) \
    (((relation) >> ((row) * (set_size) + (col))) & 1)

extern bool (*const property_functions[])(uint64_t, uint8_t);
extern const char *property_names[NUMBER_OF_PROPERTIES];

static inline bool reflexivity(uint64_t relation, uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++)
        if (!GET(relation, set_size, i, i)) return false;

    return true;
}

static inline bool irreflexivity(uint64_t relation, uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++)
        if (GET(relation, set_size, i, i)) return false;

    return true;
}

static inline bool symmetry(uint64_t relation, uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++)
        for (uint8_t j = i + 1; j < set_size; j++)
            if (GET(relation, set_size, i, j) != GET(relation, set_size, j, i))
                return false;

    return true;
}

static inline bool asymmetry(uint64_t relation, uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++) {
        if (GET(relation, set_size, i, i)) return false;

        for (uint8_t j = i + 1; j < set_size; j++)
            if (GET(relation, set_size, i, j) && GET(relation, set_size, j, i))
                return false;
    }

    return true;
}

static inline bool antisymmetry(uint64_t relation, uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++)
        for (uint8_t j = i + 1; j < set_size; j++)
            if (GET(relation, set_size, i, j) && GET(relation, set_size, j, i))
                return false;

    return true;
}

static inline bool transitivity(uint64_t relation, uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++)
        for (uint8_t j = 0; j < set_size; j++)
            for (uint8_t k = 0; k < set_size; k++)
                if (GET(relation, set_size, i, j) &&
                    GET(relation, set_size, j, k) &&
                    !GET(relation, set_size, i, k))
                    return false;

    return true;
}

static inline bool antitransitivity(uint64_t relation, uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++)
        for (uint8_t j = 0; j < set_size; j++)
            for (uint8_t k = 0; k < set_size; k++)
                if (GET(relation, set_size, i, j) &&
                    GET(relation, set_size, j, k) &&
                    GET(relation, set_size, i, k))
                    return false;

    return true;
}

static inline bool totality(uint64_t relation, uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++) {
        if (!GET(relation, set_size, i, i)) return false;

        for (uint8_t j = i + 1; j < set_size; j++)
            if (!GET(relation, set_size, i, j) &&
                !GET(relation, set_size, j, i))
                return false;
    }

    return true;
}

static inline bool trichotomy(uint64_t relation, uint8_t set_size) {
    for (uint8_t i = 0; i < set_size; i++)
        for (uint8_t j = i + 1; j < set_size; j++)
            if (!GET(relation, set_size, i, j) &&
                !GET(relation, set_size, j, i))
                return false;

    return true;
}

#endif
