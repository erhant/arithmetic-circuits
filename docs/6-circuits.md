# Designing Circuits

Now that we know what arithmetic circuits are, let's see how we can design them. We don't usually do these by hand these days; instead, a tool does that for us. But it's still important to understand how they work.

## Multiplication

Multiplication is the most basic operation in circuits. We can represent multiplication as a gate that takes two inputs and outputs their product. We already know that this is the most primitive constraint that we can have in R1CS.

Let's go one more step, and consider a proof such as "I know $k$ factors of a number $n$".

$$
n = x_1 \times x_2 \times \ldots \times x_k
$$

This can be represented as a circuit with $k-1$ multiplication gates. We can represent this as:

- $w_1 = x_1 \times x_2$
- $w_2 = w_1 \times x_3$
- $\ldots$
- $w_k = w_{k-1} \times x_k$

Do you see any problems with this? There are two problems, also called "soundness errors" with this approach:

- One can provide $1$ as a factor, it is not checked.
- One can provide pairs of inverses such as $a$ and $a^{-1}$ as factors which cancel out.

To fix it, we should check that $n$ is equal to $w_k$ alone, but not to any of the intermediate values.

## Bits & Logic Gates

Boolean circuits are a great way to represent computations. They are made up of logic gates that take in boolean values (`0` and `1`) and output boolean values. Let us consider two inputs $b_0$ and $b_1$.

- **NOT**: $1 - b_0$
- **AND**: $b_0 \times b_1$
- **OR**: $b_0 + b_1 - b_0 \times b_1$
- **XOR**: $b_0 + b_1 - 2 \times b_0 \times b_1$

Note that these are defined over a field, so they will result in some non-sense if the values are not 0 or 1. We need to constrain that as well:

- **BIT**: $b \times b = b$

## Multiplexing

How do we have "control-flow" in circuits, i.e. the thing that corresponds to if-else statements? We can use a multiplexer (MUX) for that. A MUX takes in a control bit and two inputs, and outputs one of the inputs based on the control bit.

Suppose you have two inputs $t$ (for true/1) and $f$ (for false/0), and a control bit $c$. The output $o$ of the multiplexer is given by:

$$
o = f \times (1 - c) + t \times c
$$

Higher multiplexers can be built using these basic ones, grouped together! For example, a 4-to-1 multiplexer can be built using 3 different 2-to-1 multiplexers.

## Comparators

Here is when things start to get interesting. Comparators are circuits that compare two numbers and output a boolean value.

### Is Zero: $a = 0$

First we need to check if a number is zero. The trick here is to observe that a number is non-zero if it has an inverse, and it is has an inverse then their multiplication should be 1.

We will get help from the Circom code for this:

```cs
template IsZero() {
  signal input in;
  signal output out;

  signal inv <-- in != 0 ? 1 / in : 0;
  out <== 1 - (in * inv);

  in * out === 0;
}
```

Here, `inv` is a signal that we introduce from outside the circuit (also called a **hint**), but it is constrained in some way. Let us look at our constraints, for input $i$ and output $o$, with inverse shown as $i^{-1}$:

- $w = i \times i^{-1}$
- $o = 1 - w$
- $0 = i \times o$

Again, it is important to note that $i^{-1}$ is not constrained beside its presence in $w$, and if $i$ is 0 then $i^{-1}$ can technically be anything. If $i$ is non-zero, then $i^{-1}$ is constrained to be the inverse of $i$.

### Is Equal: $a = b$

To check if two numbers are equal, we can simply subtract them and check if the result is zero. We use the zero check circuit we just designed above to do this.

$$
\text{IsEqual}(a, b) = \text{IsZero}(a - b)
$$

### Less Than: $a < b$

Comparing two numbers is a bit more involved. First observation is that if we can do $a < b$ over integers, then we can derive all other comparators:

- $a > b \implies b < a$
- $a \leq b \implies a < b+1$
- $a \geq b \implies b \leq a \implies b < a+1$

So now, how do we do $a < b$ in a circuit? Again, let us look at the Circom code for this:

```cs
template Num2Bits(n) {
  assert(n < 254);
  signal input in;
  signal output out[n];

  var lc = 0;
  var bit_value = 1;

  for (var i = 0; i < n; i++) {
    out[i] <-- (in >> i) & 1;
    out[i] * out[i] === out[i];

    lc += out[i] * bit_value;
    bit_value <<= 1;
  }

  lc === in;
}

template LessThan(n) {
  assert(n <= 252);
  signal input in[2];
  signal output out;

  component toBits = Num2Bits(n+1);
  toBits.in <== ((1 << n) + in[0]) - in[1];

  out <== 1 - toBits.out[n];
}
```

Here, `Num2Bits` is a template that converts a number to its binary representation with the given amount of bits. How this happens is that we give **hint** for each bit of the number, and then we sum them up to get the number back. Here is what the main constraint looks like for this:

$$
\text{in} = \sum_{i=0}^{n} \text{out}_i \times 2^i
$$

Now, with $a < b$, we first assume that both numbers are at most $n$ bits, and then we compute the expression: $2^n + (a - b)$. The answer of $a < b$ now lies within the most significant bit (MSB) of the result:

- If MSB is 1, then $a >= b$ because there was no borrowed bits.
- If MSB is 0, then $a < b$ because the extra bit had to be borrowed!

Let's give an example on this over 3-bit numbers. Here is a case with $a = 3$ and $b=2$:

```py
1 0 1 1 = 11 (3 + 2^3)
  0 1 0 = 2
_______   -
1 0 0 1 = 9
#--> msb is 1, so 3 > 2
```

Now here is a case with $a = 3$ and $b = 4$:

```py
1 0 1 1 = 11 (3 + 2^3)
  1 0 0 = 4
_______   -
0 1 1 1 = 7
#--> msb is 0, so 3 < 4
```

## Arrays

For the last part, we will take a little tour over managing arrays in circuits. An array simply stands for a collection of inputs, but the important thing is that the length of the array is fixed & must be known at the time of creating the circuit!

### Reading an array

Suppose you have an array `A` with $n$ elements, and you want to read the $i$-th element of it. The way to do this is rather costly in a circuit: we could loop over all elements and select the $i$-th one by multiplying all elements with an $\text{IsEqual(i, j)}$ where $i$ is the element you want and $j$ is the index of that loop.

```py
A[i] = A[0]   * 0
     + A[1]   * 0
     + ...
     + A[i]   * 1
     + ...
     + A[n-1] * 0
```

This is also known as "linear scan" or a "Quin selector".

> You could use an $n$-to-$1$ multiplexer built over smaller multiplexers to select the $i$-th element.

### Writing to an array

Writing to an array is similar, but around 2x more expensive because we can't really "modify" a signal in a circuit. We can only create new signals that are constrained to be equal to the old ones, except the one that we are writing to.

Suppose we are writing `x` at index `i` of some array `A` and the result is an array `B`, this operation would look like:

```py
  B[0] = A[0]   * 1 + x * 0
  B[1] = A[1]   * 1 + x * 0
   ... = ...
  B[i] = A[i]   * 0 + x * 1
   ... = ...
B[n-1] = A[n-1] * 1 + x * 0
```

[➡️ HOME](../README.md)
