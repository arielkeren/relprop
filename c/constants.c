#include "constants.h"

#include <stdbool.h>
#include <stdint.h>

#include "properties.h"

const char *PROPERTY_NAMES[NUMBER_OF_PROPERTIES] = {"Antisymmetry",
                                                    "Antitransitivity",
                                                    "Asymmetry",
                                                    "Coreflexivity",
                                                    "Density",
                                                    "Irreflexivity",
                                                    "LeftEuclideanness",
                                                    "LeftQuasiReflexivity",
                                                    "QuasiReflexivity",
                                                    "Reflexivity",
                                                    "RightEuclideanness",
                                                    "RightQuasiReflexivity",
                                                    "StrictDensity",
                                                    "Symmetry",
                                                    "Totality",
                                                    "Transitivity",
                                                    "Trichotomy"};

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
