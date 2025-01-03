# ModaPalas Blackboard | Cryptography II: Arithmetic Circuits

<a href="./.github/workflows/test.yml" target="_blank">
  <img alt="Workflow: Tests" src="https://github.com/erhant/arithmetic-circuits/actions/workflows/tests.yml/badge.svg">
</a>

This repository contains the material for the event [ModaPalas Blackboard | Cryptography II](https://lu.ma/c6wrtj5b?tk=dJKwGa) lecture "Arithmetic Circuits" by [erhant](https://github.com/erhant).

## Lecture Notes

We have lecture notes as follows:

1. [Introduction](./docs/1-intro.md)
2. [Groups](./docs/2-groups.md)
3. [Usage in Zero Knowledge Cryptography](./docs/3-usage.md)
4. [Rank-1 Constraint Systems](./docs/4-r1cs.md)
5. [Quadratic Arithmetic Programs](./docs/5-qap.md)
6. [Designing Circuits](./docs/6-circuits.md)

If you have `mdbook` installed with Mermaid preprocessor, you can also open this in a book via:

```sh
mdbook serve --open
```

## Implementations

We also have some pieces of code:

- [R1CS](./src/r1cs.rs): a simple implementation of Rank-1 Constraint System.
- [QAP](./src/qap.rs): a simple implementation of Quadratic Arithmetic Program, connected to the R1CS implementation.
- [Circuits](./src/circuits/mod.rs): a "wire" implementation and some circuit examples over it.

You can run some tests and see the results by running:

```sh
cargo test -- --show-output
```

We have an example R1CS & QAP implementation that you can run, which shows you all the results:

```rs
// A interpolations:
        A_0(x) = 51*x^2 + 41*x + 5
        A_1(x) = x^2 + 93*x + 4
        A_2(x) = 96*x^2 + 4*x + 94
        A_3(x) = 49*x^2 + 47*x + 1
// A evaluations:
        A_0(1) = 0      A_0(2) = 0      A_0(3) = 5
        A_1(1) = 1      A_1(2) = 0      A_1(3) = 1
        A_2(1) = 0      A_2(2) = 1      A_2(3) = 0
        A_3(1) = 0      A_3(2) = 0      A_3(3) = 1

// B interpolations:
        B_0(x) = 0
        B_1(x) = 48*x^2 + 50*x
        B_2(x) = 0
        B_3(x) = 0
// B evaluations:
        B_0(1) = 0      B_0(2) = 0      B_0(3) = 0
        B_1(1) = 1      B_1(2) = 1      B_1(3) = 0
        B_2(1) = 0      B_2(2) = 0      B_2(3) = 0
        B_3(1) = 0      B_3(2) = 0      B_3(3) = 0

// C interpolations:
        C_0(x) = 0
        C_1(x) = 0
        C_2(x) = 49*x^2 + 46*x + 3
        C_3(x) = 96*x^2 + 4*x + 94
// C evaluations:
        C_0(1) = 0      C_0(2) = 0      C_0(3) = 0
        C_1(1) = 0      C_1(2) = 0      C_1(3) = 0
        C_2(1) = 1      C_2(2) = 0      C_2(3) = 0
        C_3(1) = 0      C_3(2) = 1      C_3(3) = 0

// P(x) built from QAP for a valid instance
P(x) = 82*x^4 + 81*x^3 + 83*x^2 + 88*x + 54
P(x) / T(x) = 82*x + 88
```

We can also design circuits with the existing gates (see [this example](./tests/circuits_test.rs)) and then print a wire's lavel to see its gates, everything is composed of `+` and `*` only!

```py
# for equal wires a, b in a field of order 97
a == b is given by (1+(96*((a+(96*b))*0))): 1
```

## See Also

- [Ying Tong Lai - Modern ZK Cryptography: Lecture 7 (MIT IAP 2023)](https://assets.super.so/9c1ce0ba-bad4-4680-8c65-3a46532bf44a/files/e11309fb-7356-42ad-9c78-565341abd80d.pdf)
- [0xPARC - R1CS Explainer](https://learn.0xparc.org/materials/circom/additional-learning-resources/r1cs%20explainer/)
- [Vitalik Buterin - QAP: Zero-to-Hero](https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649)
- [RareSkills - R1CS](https://www.rareskills.io/post/rank-1-constraint-system)
- [erhant - notes from ZKP MOOC Lecture 3: Programming ZKPs](https://crypto.erhant.me/zklearning/programming-zkps.html)
- [Risen Crypto - R1CS and QAP](https://risencrypto.github.io/R1CSQAP/)
