# ModaPalas Blackboard | Cryptography II: Arithmetic Circuits

This repository contains the material for the event [ModaPalas Blackboard | Cryptography II](https://lu.ma/c6wrtj5b?tk=dJKwGa) lecture "Arithmetic Circuits" by [erhant](https://github.com/erhant).

## Lecture Notes

We have lecture notes as follows:

1. [Introduction](./docs/1-ontro.md)
2. [Groups](./docs/2-groups.md)
3. [Usage in Zero Knowledge Cryptography](./docs/3-usage.md)
4. [Rank-1 Constraint Systems](./docs/4-r1cs.md)
5. [Quadratic Arithmetic Programs](./docs/5-qap.md)
6. [Designing Circuits](./docs/6-circuits.md)

## Implementations

We also have some pieces of code:

- [R1CS](./src/r1cs.rs): a simple implementation of Rank-1 Constraint System.
- [QAP](./src/qap.rs): a simple implementation of Quadratic Arithmetic Program, connected to the R1CS implementation.
- [Circuits](./src/circuits.rs): a "wire" implementation and some circuit examples over it.

You can run some tests and see the results by running:

```sh
cargo test -- --show-output
```

In particular, you can design circuits with the existing gates (see [this example](./tests/circuits_test.rs)) and then print a wire that you have to see its gates, everything is composd of `+` and `*` gates only!

## See Also

- [Ying Tong Lai - Modern ZK Cryptography: Lecture 7 (MIT IAP 2023)](https://assets.super.so/9c1ce0ba-bad4-4680-8c65-3a46532bf44a/files/e11309fb-7356-42ad-9c78-565341abd80d.pdf)
- [0xPARC - R1CS Explainer](https://learn.0xparc.org/materials/circom/additional-learning-resources/r1cs%20explainer/)
- [Vitalik Buterin - QAP: Zero-to-Hero](https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649)
- [RareSkills - R1CS](https://www.rareskills.io/post/rank-1-constraint-system)
- [erhant - notes from ZKP MOOC Lecture 3: Programming ZKPs](https://crypto.erhant.me/zklearning/programming-zkps.html)
- [Risen Crypto - R1CS and QAP](https://risencrypto.github.io/R1CSQAP/)
