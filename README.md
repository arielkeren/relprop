# RelProp

**RelProp** is a high-performance, cross-language tool for enumerating and analyzing binary relations on finite sets. It computes, for each set size, how many relations satisfy a wide range of mathematical properties (such as reflexivity, symmetry, transitivity, etc.), and provides both raw counts and percentages. RelProp is implemented in C, Rust, and Go, and includes utilities for result validation, visualization, and statistical analysis.

---

## Features

- **Efficient enumeration** of all binary relations for any set size (subject to computational limits).
- **Checks 17 standard properties** of binary relations, including combinations and rare properties.
- **Parallelized computation** for high performance (OpenMP in C, Rayon in Rust).
- **CSV output** of results for easy analysis and visualization.
- **Visualization tools** (Python/Jupyter) for plotting trends and correlations.
- **Validation utilities** (Go) to cross-check computed results against closed formulas.
- **Highly modular codebase**: C, Rust, and Go implementations, with shared logic and test suites.
- **Cross-platform Makefile** for easy building and cleaning (Windows-focused, but portable).
- **Mathematical background and references** included for each property.

---

## Output

- **CSV file** (`results.csv`) with columns for set size, time, total relations, and for each property: count and percentage.
- **Visualization**: Stacked area plots, correlation matrices, and more.
- **Validation**: Go tool checks computed counts against closed formulas.

---

### Example CSV Output

| Set | Time | Total | Reflexivity_Count | Reflexivity_Pct | Symmetry_Count | Symmetry_Pct | Transitivity_Count | Transitivity_Pct |
| --- | ---- | ----- | ----------------- | --------------- | -------------- | ------------ | ------------------ | ---------------- |
| 2   | 0.01 | 16    | 4                 | 25.00           | 8              | 50.00        | 8                  | 50.00            |
| 3   | 0.02 | 512   | 64                | 12.50           | 64             | 12.50        | 29                 | 5.66             |
| 4   | 0.10 | 65536 | 4096              | 6.25            | 512            | 0.78         | 355                | 0.54             |

## Mathematical Background

A **binary relation** on a set $S$ of finite cardinality (size) $n$ is a subset of the _cartesian multiplication_ of $S$ with itself. There are $2$ ^ $n^2$ possible binary relations on $S$. Each relation can be represented as an $n$ by $n$ Boolean matrix. In this project, for faster checking, bitwise operations are used, along with each relation being represented as a fixed array of unsigned integers.

**RelProp** systematically enumerates all such relations for a given $n$, and checks which properties each relation satisfies.

---

## Properties Checked

| Property                    | Definition                                                                           | Closed Formula (Count)   |
| --------------------------- | ------------------------------------------------------------------------------------ | ------------------------ |
| **Reflexivity**             | For all x: (x, x) is in R                                                            | 2^(n²-n)                 |
| **Irreflexivity**           | For all x: (x, x) is not in R                                                        | 2^(n²-n)                 |
| **Symmetry**                | For all x, y: if (x, y) is in R, then (y, x) is in R                                 | 2^(n(n+1)/2)             |
| **Antisymmetry**            | For all x ≠ y: if (x, y) and (y, x) are in R, then x = y                             | 3^{n(n-1)/2} × 2^n       |
| **Asymmetry**               | For all x, y: if (x, y) is in R, then (y, x) is not in R                             | 2^{n(n-1)/2}             |
| **Coreflexivity**           | For all x, y: if (x, y) is in R, then x = y                                          | 2^n                      |
| **Trichotomy**              | For all x ≠ y: exactly one of (x, y) or (y, x) is in R                               | 3^{n(n-1)/2}             |
| **Totality**                | For all x ≠ y: (x, y) is in R or (y, x) is in R                                      | 3^{n(n-1)/2} × 2^n       |
| **Transitivity**            | For all x, y, z: if (x, y) and (y, z) are in R, then (x, z) is in R                  | No simple closed formula |
| **Antitransitivity**        | For all x, y, z: if (x, y) and (y, z) are in R, then (x, z) is not in R              | No simple closed formula |
| **Density**                 | For all x, y: if (x, y) is in R, there exists z such that (x, z) and (z, y) are in R | No simple closed formula |
| **Strict Density**          | Like density, but z ≠ x and z ≠ y                                                    | No simple closed formula |
| **Left Euclideanness**      | For all x, y, z: if (y, x) and (z, x) are in R, then (y, z) is in R                  | No simple closed formula |
| **Right Euclideanness**     | For all x, y, z: if (x, y) and (x, z) are in R, then (y, z) is in R                  | No simple closed formula |
| **Left Quasi-Reflexivity**  | For all x, y: if (x, y) is in R, then (x, x) is in R                                 | No simple closed formula |
| **Right Quasi-Reflexivity** | For all x, y: if (x, y) is in R, then (y, y) is in R                                 | No simple closed formula |
| **Quasi-Reflexivity**       | For all x, y: if (x, y) is in R, then (x, x) or (y, y) is in R                       | No simple closed formula |

---

## Usage

### **C Version (Highly Optimized)**

```sh
cd highly_optimized
make
./relations --min 2 --max 5
# Or
./relations -m 2 -M 5
```

### **Rust Version (Full Features)**

```sh
cd full_features
cargo run --release -- --min 2 --max 5 --properties symmetry antisymmetry transitivity
# Or
cargo run --release -- -m 2 -M 5 -p symmetry antisymmetry transitivity
```

### **Go Tests (Testing CSV)**

```sh
cd results_test
go run .
```

### **Python Jupyter Notebook (Visualization)**

Open the provided Jupyter notebook to visualize and analyze the CSV output.
