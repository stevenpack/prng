# prng
A pseudo-random number generator.

Seeded with time by default.
Allocation address also an option.

Created because my first wasm project needed random numbers and calls to from_entropy fail in wasm, as they're not implemented by the host at the time of writing.
