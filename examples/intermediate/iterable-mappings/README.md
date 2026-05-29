# Iterable Mapping Utilities

This example shows how to work with Soroban `Vec<u32>` collections using a few small helper-style operations:

- `filter_by(values, threshold)` keeps only values that meet the threshold.
- `map_by(values, offset)` applies a fixed transformation to every element.
- `reduce_sum(values)` adds the whole collection in one linear pass.

## Why this matters

These helpers are intentionally simple:

- they use one pass over the input vector,
- they avoid nested loops and repeated allocations,
- and their gas cost grows predictably with the size of the input.

## Suggested usage

1. Start with a small, fixed-size input vector.
2. Apply `filter_by` when you want to keep only relevant values.
3. Use `map_by` for a deterministic transformation.
4. Finish with `reduce_sum` to calculate a single aggregate value.
