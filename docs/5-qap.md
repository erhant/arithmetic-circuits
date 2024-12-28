# Quadratic Arithmetic Program (QAP)

Going from an R1CS to a QAP is a bit more involved than going from an arithmetic circuit to an R1CS.

A Quadratic Arithmetic Program (QAP) is a system of equations that can be used to represent a computation. Consider an R1CS $R$ where we had $n$ constraints and $m$ variables, with $A, B, C$ being the matrices representing the constraints. We can represent this as a QAP as follows:

$$
\begin{align*}
QAP(R) = \{
  T \in \mathbb{F}[x],
  \{
    A_i, B_i, C_i \in \mathbb{F}[x]
  \}_{j=1}^{m}

\}
\end{align*}
$$

where each $A, B, C$ polynomial is at most degree-$(n-1)$, and the target polynomial is degree-$n$. For each constraint a random variable from the field is picked, resulting in $n$ random values $\{r_1, r_2, \ldots, r_n\}$. The target polynomial $T(x)$ is defined as:

$$
T(x) = \prod_{i=1}^{n} (x - r_i)
$$

> In other words, we have a polynomial for each variable's coefficient in $A, B, C$ that we can select with the evaluation point, e.g. $A_j(m_i)$ selects the coefficient for the $j$-th variable in $i$-th constraint w.r.t $A$ matrix.
>
> The target polynomial is such that it evaluates to zero at each of the random points $r_i$.

Constructing $A, B, C$ polynomials use something called [Lagrange Interpolation](https://mathworld.wolfram.com/LagrangeInterpolatingPolynomial.html). Basically, if you have pairs of values $\{(x_1, y_1), (x_2, y_2), \ldots, (x_n, y_n)\}$ this technique allows you to find a polynomial that passes through all these points, and it is guaranteed to be unique for degree $k-1$.

The trick of QAP is the following observation: for $m$ variables $\{x_1, x_2, \ldots, x_m\}$, we can compute the polynomial $P(x)$:

$$
P(x) =
\sum_{i=1}^{m} A_i(x) \cdot x_i \circ \sum_{i=1}^{m} B_i(x) \cdot x_i - \sum_{i=1}^{m} C_i(x) \cdot x_i
$$

If $x$ variables were indeed satisfying the R1CS in the first place, then this polynomial $P(x)$ must be divisible by the target polynomial $T(x)$.

Using a single polynomial, we were able to capture the entire computation of the circuit. This is the power of QAPs!

> Recall our R1CS definition of a single constraint from before, and notice how similar it looks!
>
> $$
> \begin{align*}
> \sum_{i=1}^{m} a_i \cdot x_i \circ \sum_{i=1}^{m} b_i \cdot x_i - \sum_{i=1}^{m} c_i \cdot x_i
> \end{align*}
> $$

[➡️ NEXT SECTION](./6-circuits.md)
