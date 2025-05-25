# AES Evaluation over TFHE

This repository presents a detailed implementation of the Advanced Encryption Standard (AES) using Fully Homomorphic Encryption (FHE) under the TFHE scheme. The project investigates and compares two distinct approaches to enable efficient encrypted AES computation:

1. **Full-LUT Based Evaluation**: All AES layers are implemented using Lookup Tables (LUTs) evaluated through programmable bootstrapping.
2. **Hippogryph Hybrid Approach**: Combines LUTs for non-linear operations (e.g., SubBytes) with TFHE’s native arithmetic for linear operations (e.g., XOR, MixColumns), achieving execution under 4 seconds.

This work is driven by the need for practical transciphering solutions, where encrypted data can be processed directly without compromising security. It was an assignment as part of the course CSL-513(Information and Network Security) at IIT Roorkee in our 8th semester.

---

## Repository Links

- Full-LUT Based Implementation: https://github.com/daphnetrm/Benchmark-of-AES-Evaluation-with-TFHE
- Hippogryph (Hybrid Implementation): [https://github.com/CryptoExperts/Hippogryph](https://github.com/CryptoExperts/Hippogryph)

---

## Background

TFHE (Fast Fully Homomorphic Encryption over the Torus) is a homomorphic encryption scheme supporting programmable bootstrapping (PBS), enabling efficient evaluation of functions over encrypted data. It is particularly optimized for low-precision values and supports bootstrapping-based LUT evaluation.

Evaluating AES under FHE introduces significant challenges due to:

- High cost of non-linear operations like SubBytes and MixColumns
- Exponential noise growth across rounds
- Repeated bootstrapping requirements

This project explores techniques to mitigate these challenges, including:

- **Multi-Value Bootstrapping (MVB)** to perform multiple LUT evaluations with one bootstrapping
- **Tree-Based LUT Decomposition** to evaluate XOR and SubBytes efficiently
- **Domain Switching (E2 ↔ E17)** between Boolean and arithmetic representations

---

## Requirements

### Full-LUT Implementation

- Language: C++
- Compiler: `g++` with OpenMP support
- Libraries:
  - TFHE or compatible libraries for encrypted arithmetic

### Hippogryph

- Language: Rust
- Toolchain:
  - Rust >= 1.65
  - Cargo (Rust package manager)

---

## Build and Execution Instructions

### Full-LUT Implementation

## Build and Execution Instructions

### Full-LUT Based AES Evaluation

This implementation evaluates all AES operations using programmable bootstrapping and LUTs. It is based on the original work published at:

[https://github.com/daphnetrm/Benchmark-of-AES-Evaluation-with-TFHE](https://github.com/daphnetrm/Benchmark-of-AES-Evaluation-with-TFHE)

```bash
git clone https://github.com/daphnetrm/Benchmark-of-AES-Evaluation-with-TFHE
cd Benchmark-of-AES-Evaluation-with-TFHE

# Create a build directory and configure
mkdir build && cd build
cmake ..
make -j

# Run the benchmark
./aes_tfhe_benchmark
```
### Hippogryph
```bash
git clone https://github.com/CryptoExperts/Hippogryph
cd Hippogryph/hippogriph

# Build in release mode
cargo build --release

# Run the optimized AES-FHE pipeline
cargo run --release
```
This is the optimized version combining the Hippogryph hybrid approach with custom improvements. It is available in the Hippogryph-main folder of the following repository:

https://github.com/pavansai444/CSL-513-Project/
```bash
git clone https://github.com/pavansai444/CSL-513-Project
cd CSL-513-Project/Hippogryph-main/hippogriph

# Build in release mode using Cargo
cargo build --release

# Execute the optimized AES-FHE pipeline
cargo run --release

```
## Implementation Details

### Full-LUT AES Evaluation

- All AES transformations are implemented using programmable bootstrapping.
- XOR and GF(256) multiplications are expressed using 16×16 LUTs via basis-16 decomposition.
- SubBytes is computed via two 4-bit LUTs using Multi-Value Bootstrapping (MVB) to reduce bootstrapping steps.
- Parallel execution using OpenMP reduces execution time from 220 seconds to approximately 36.47 seconds with 32 threads.

### Hippogryph (Sub-4s Execution)

- **SubBytes**: Implemented using LUTs with E17 encoding and tree-based MVB.
- **ShiftRows**: Realized as a ciphertext permutation.
- **MixColumns and AddRoundKey**: Optimized using native TFHE additions over \(\mathbb{Z}_2\).
- **Decomposer**: Converts from E17 to E2 using a single PBS per nibble with MVB.
- **Recomposer**: Converts from E2 back to E17 using one PBS per bit.
- **Negacyclicity**: Resolved using odd modulus \(\mathbb{Z}_{17}\) instead of padding-based approaches.

---

## Code Optimizations done by us

- Eliminated dynamic memory allocation in performance-critical functions.
- Preloaded static AES S-box and circuit structures at compile time.
- Removed runtime file I/O by embedding logic structures in memory.
- Parallelized `SubBytes`, `MixColumns`, and `AddRoundKey` operations.
- Merged `AddRoundKey` and `SubBytes` to reduce dependencies and improve concurrency.

---

## Experimental Results

| Approach                 | Cores/Threads | Runtime (seconds) |
|--------------------------|----------------|-------------------|
| Full-LUT (serial)        | 1              | 220.00            |
| Full-LUT (parallelized)  | 32             | 36.47             |
| Hippogryph (initial)     | 8              | 4.01              |
| Hippogryph (optimized)   | 8              | 3.70              |

---

## Output Verification

Correctness is verified by comparing the FHE-decrypted AES ciphertext with the known AES result. An example output:
```bash
Output: 69 c4 e0 d8 6a 7b 04 30 d8 cd b7 80 70 b4 c5 5a
Expected: 69 c4 e0 d8 6a 7b 04 30 d8 cd b7 80 70 b4 c5 5a
```

Each stage of the pipeline logs its intermediate and final output for validation.

---

## Contributors

This work was completed as part of the CSL-513 course project by the following contributors:

- **Balaga Pavan Sai** (Enrollment No. 21114025)  
- **Bandi Pavan Karthik** (Enrollment No. 21114026)  
- **Male Jithendra** (Enrollment No. 21114055)  
- **Taviti Venkata Manikanta** (Enrollment No. 21114105)  
- **Uppala Vivek Narayan** (Enrollment No. 21114108)

---

## License

This project is licensed under the MIT License. You are free to use, modify, and distribute this software for academic, research, or commercial purposes, provided the license file is included with all copies and substantial portions of the software.

**Note**: External dependencies and linked repositories (e.g., CryptoExperts/Hippogryph) may have their own licensing terms. Please refer to their respective LICENSE files.
<!--Made some modifications to use preloaded circuit operations instead of loading from external files in Linear Circuit.-->
