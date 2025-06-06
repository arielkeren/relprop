#ifndef PROPERTIES_H
#define PROPERTIES_H

#include <stdbool.h>
#include <stdint.h>

#define NUMBER_OF_PROPERTIES 17

#define GET(relation, row, col) (((relation[row]) >> (col)) & 1)

extern bool (*const property_functions[NUMBER_OF_PROPERTIES])(uint8_t[],
                                                              uint8_t);
extern const char *property_names[NUMBER_OF_PROPERTIES];

bool antisymmetry(uint8_t relation[], uint8_t set_size);
bool antitransitivity(uint8_t relation[], uint8_t set_size);
bool asymmetry(uint8_t relation[], uint8_t set_size);
bool coreflexivity(uint8_t relation[], uint8_t set_size);
bool density(uint8_t relation[], uint8_t set_size);
bool irreflexivity(uint8_t relation[], uint8_t set_size);
bool left_euclideanness(uint8_t relation[], uint8_t set_size);
bool left_quasi_reflexivity(uint8_t relation[], uint8_t set_size);
bool quasi_reflexivity(uint8_t relation[], uint8_t set_size);
bool reflexivity(uint8_t relation[], uint8_t set_size);
bool right_euclideanness(uint8_t relation[], uint8_t set_size);
bool right_quasi_reflexivity(uint8_t relation[], uint8_t set_size);
bool strict_density(uint8_t relation[], uint8_t set_size);
bool symmetry(uint8_t relation[], uint8_t set_size);
bool totality(uint8_t relation[], uint8_t set_size);
bool transitivity(uint8_t relation[], uint8_t set_size);
bool trichotomy(uint8_t relation[], uint8_t set_size);

#endif
