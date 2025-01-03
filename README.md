# JIT Calculator

A [Cranelift](https://cranelift.dev/)-backed, JIT-compiled, very basic calculator, made in response to a [nerd snipe](https://ochagavia.nl/blog/the-jit-calculator-challenge/) from Adolfo Ochagavía.

## Results

Benchmark results are from running the program `+*-/--**++//`:

| Implementation | CPU       | Runtime |
|----------------|-----------|--------:|
| Interpreter    | AMD 5700X | 17.0 ms |
| JIT            | AMD 5700X |  2.9 ms |
