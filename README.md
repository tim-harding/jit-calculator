# JIT Calculator

A [Cranelift](https://cranelift.dev/)-backed, JIT-compiled, very basic calculator, made in response to a [nerd snipe](https://ochagavia.nl/blog/the-jit-calculator-challenge/) from Adolfo Ochagavía.

## Results

Benchmark results are from running the program `+*-/--**++//`:

| Implementation | Runtime |
|----------------|--------:|
| Interpreter    | 17.0 ms |
| JIT            |  2.9 ms |
