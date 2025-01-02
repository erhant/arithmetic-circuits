# Quadratic Arithmetic Program (QAP)

Going from an R1CS to a QAP is a bit more involved than going from an arithmetic circuit to an R1CS.

A Quadratic Arithmetic Program (QAP) is a system of equations that can be used to represent a computation. Consider an R1CS $R$ where we had $n$ constraints and $m$ variables, with $A, B, C$ being the matrices representing the constraints. We can represent this as a QAP as follows:

$$
\begin{align*}
QAP(R) = \left\lbrace
  T \in \mathbb{F}[x],
  \lbrace
    A_i, B_i, C_i \in \mathbb{F}[x]
  \rbrace _{j=1}^{m}
\right\rbrace
\end{align*}
$$

where each $A, B, C$ polynomial is at most degree-$(n-1)$, and the target polynomial is degree-$n$, and there are $m$ polynomials (one for each variable). For each constraint a random variable from the field is picked, resulting in $n$ random values $\lbrace r_1, r_2, \ldots, r_n \rbrace$. The target polynomial $T(x)$ is defined as:

$$
T(x) = \prod_{i=1}^{n} (x - r_i)
$$

> In other words, we have a polynomial for each variable's coefficient in $A, B, C$ that we can select with the evaluation point, e.g. $A_j(m_i)$ selects the coefficient for the $j$-th variable in $i$-th constraint w.r.t $A$ matrix.
>
> The target polynomial is such that it evaluates to zero at each of the random points $r_i$.

Constructing $A, B, C$ polynomials use something called [Lagrange Interpolation](https://mathworld.wolfram.com/LagrangeInterpolatingPolynomial.html). Basically, if you have pairs of values $\lbrace (x_1, y_1), (x_2, y_2), \ldots, (x_n, y_n) \rbrace$ this technique allows you to find a polynomial that passes through all these points, and it is guaranteed to be unique for degree $k-1$.

The trick of QAP is the following observation: for $m$ variables $\lbrace x_1, x_2, \ldots, x_m \rbrace$ (with $x_1 = 1$), we can compute the polynomial $P(x)$:

$$
P(x) =
\sum_{i=1}^{m} A_i(x) \cdot x_i \times \sum_{i=1}^{m} B_i(x) \cdot x_i - \sum_{i=1}^{m} C_i(x) \cdot x_i
$$

If $x$ variables were indeed satisfying the R1CS, then this polynomial $P(x)$ when evaluated over all evaluation points $\lbrace m_1, m_2, \ldots, m_n \rbrace$ will result in 0.

There is a better way to check this instead of evaluating at all points, and this is where the target polynomial comes in! If $P(x)$ is indeed evaluating to 0 over all evaluation points, then it **must** be divisible by the target polynomial $T(x)$. So, we can simply do a division check to see if its satisfying or not.

Using a single polynomial $P(x)$ for some instance $x$, we were able to capture the entire computation of the circuit. This is the power of QAPs!

> In some sources, instead of $A, B, C$ we use $L, R, O$ that stand for _left_, _right_ and _output_ of a gate.

## Comparing to R1CS

In R1CS, for $n$ constraints and $m$ variables we had 3 matrices $A, B, C$ of size $n \times m$. In QAP, we have $n$ polynomials $A, B, C$ each of degree at most $n-1$, meaning that they have $n$ terms at most. Basically, we were able to "squish" the columns of the matrix into a single polynomial using interpolation, where a random selector number $m_i$ is able to select the column for some constraint.

Also recall our R1CS definition of a single constraint from before, and notice how similar it looks!

$$
\begin{align*}
\sum_{i=1}^{m} a_i \cdot x_i \times \sum_{i=1}^{m} b_i \cdot x_i - \sum_{i=1}^{m} c_i \cdot x_i
\end{align*}
$$

## Where is the Zero-Knowledge?

In a zkSNARK, the prover proves that indeed $P(x)$ is divisible by $T(x)$, without really revealing the entire polynomial with the help of some polynomial commitment scheme (PCS) and some polynomial interactive oracle-proof (PIOP) along with some other cryptographic primitives, e.g. a bilinear pairing.

[➡️ NEXT SECTION](./6-circuits.md)
