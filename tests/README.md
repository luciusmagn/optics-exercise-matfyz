# Tests

- `cargo test`: all 50 tests.
- `cargo test --bin lens-calculator`: 16 function tests in `src/tests.rs`, using `calculate` and `Analysis`.
- `cargo test --test lenses`: 34 command-line tests.
- `examples/`: 23 valid inputs; `tests/invalid/`: nine invalid inputs.
- `tests/expected.tsv`: expected results; lengths in mm; `-` means unavailable.
- Numeric tolerance: functions `1e-9 * max(abs(expected), 1)`; CLI `0.00051 + 1e-12 * abs(expected)`.
