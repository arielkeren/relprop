#include "properties.h"

#include <stdbool.h>
#include <stdint.h>

bool (*const property_functions[])(uint64_t, uint8_t) = {
    reflexivity,  irreflexivity,    symmetry, asymmetry, antisymmetry,
    transitivity, antitransitivity, totality, trichotomy};

const char *property_names[NUMBER_OF_PROPERTIES] = {
    "Reflexivity",  "Irreflexivity",    "Symmetry", "Asymmetry", "Antisymmetry",
    "Transitivity", "Antitransitivity", "Totality", "Trichotomy"};
