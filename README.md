# JIT Calculator

A [Cranelift](https://cranelift.dev/)-backed, JIT-compiled, very basic calculator, made in response to a [nerd snipe](https://ochagavia.nl/blog/the-jit-calculator-challenge/) from Adolfo Ochagavía.

## Results

Benchmarked by running the benchmark programs with [Hyperfine](https://github.com/sharkdp/hyperfine).

| Implementation | Runtime          |
|----------------|------------------|
| Interpreter    | 46.8 ms ± 0.8 ms |
| JIT            | 48.5 ms ± 0.8 ms |
