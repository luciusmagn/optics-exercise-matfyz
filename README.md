# Lens calculator

Write a Rust command-line program that reads a thin-lens system from a file and
reports its focal properties. Use Rust 1.88 or newer and the standard library.
Complete `calculate` and `main` in `src/main.rs`. `Analysis` holds the results.
Use `Ok(analysis)` for a successful calculation and `Err(message)` for an error.

```sh
cargo run -- examples/internal-stop.opt
cargo test
```

## Input

Read the file named by the single command-line argument. Process records in the
order encountered by light. All lengths are in millimetres.

| Record | Parameters |
|---|---|
| `lens F D` | Nonzero focal length; positive clear diameter |
| `gap L` | Nonnegative distance through air |
| `stop D` | Positive opening diameter |

Negative focal lengths mean diverging lenses. Use finite decimal or scientific
notation and separate fields with spaces. Require exactly the stated number of
fields per record and at least one record. Record names are lowercase.

## Output

For `examples/internal-stop.opt`, print:

```text
System length: 25.000 mm
Effective focal length: 50.000 mm
Back focus: 25.000 mm
Entrance pupil diameter: 20.000 mm
F-number at infinity: f/2.500
Limiting opening: element 3
```

Use these six quantities in this order, with decimal points and at least three
decimal places. System length is the sum of the gaps. Measure back focal distance
from the final input plane, including any trailing gap. Number all records,
including gaps, starting at one. Report the first opening when aperture bounds tie.

Use the following complete lines for special results:

| Condition | Output line |
|---|---|
| Afocal system | `Effective focal length: infinite (afocal)` |
| Afocal system | `Back focus: none (afocal)` |
| Unbounded pupil | `Entrance pupil diameter: unbounded` |
| Undefined f-number | `F-number: not applicable` |
| No limiting opening | `Limiting opening: none` |

Calculate f-number for a positive effective focal length and finite entrance pupil.
Print results to stdout and errors to stderr. Include source line numbers for
malformed records and the element number for arithmetic overflow.

Exit codes: `0` success, `1` input or calculation error, `2` incorrect argument count.

## Tests

The 16 function tests call `calculate(&str) -> Result<Analysis, String>`.
The 34 integration tests run the executable.
The starter compiles; implement the program to pass the tests.

- `examples/`: 23 valid inputs, including named lens models and edge cases.
- `tests/expected.tsv`: expected optical results.
- `tests/invalid/`: nine invalid inputs.
- `tests/lenses.rs`: executable-interface tests.
- `src/tests.rs`: function tests.

## Optics reference

[Student handout](optics-overview-student.pdf) · [A4 printing](optics-overview-student-a4.pdf)
