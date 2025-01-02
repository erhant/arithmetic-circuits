# Group Theory

To understand what type of numbers flow within the wires of our arithmetic circuits, we need to understand some basic concepts from group theory. Group theory is a branch of abstract algebra that studies the algebraic structures known as groups.

> **Tl;DR** of this section if you want to jump ahead:
>
> Our numbers are elements of a finite field, which is a set equipped with two binary operations, addition and multiplication, that satisfy certain axioms.
>
> You can think of numbers $\lbrace 0, 1, 2, \ldots, p-1 \rbrace$ modulo some prime $p$ where addition and multiplication are done modulo $p$.
>
> All numbers except $0$ have a multiplicative inverse, i.e., for every number $a \neq 0$, there exists a number $b$ such that $a \times b = 1$.

## Groups

A group is a set equipped with a binary operation that combines any two elements to form a third element, satisfying certain properties. A group is shown as $(G, \cdot)$ and consists of the following components:

- A set of elements $G$.
- A binary operation (denoted as $\cdot$) that takes two elements and produces a third element.

The operation must satisfy the following properties:

- **Closure**: For any two elements $a, b \in G$, the result of the operation is also in the group: $a \cdot b \in G$. It is said that the group $G$ is _closed_ under its binary operation.

- **Identity**: There exists an element $e \in G$, called the identity element, such that for any element $a \in G$, the operation $a \cdot e = e \cdot a = a$.

- **Inverse**: For every element $a \in G$, there exists an element $b \in G$, called the inverse of a, such that $a \cdot b = b \cdot a = e$. The inverse of $a$ is denoted as $a^{-1}$.

- **Associativity**: For any three elements $a, b, c \in G$, the operation is associative, meaning $(a \cdot b) \cdot c = a \cdot (b \cdot c)$. This property ensures that the order of operations does not matter.

There is an additional property as well, called the **commutative property** or **abelian property**. A group is said to be _Abelian_ if the binary operation is commutative, meaning $a \cdot b = b \cdot a$ for all elements $a, b \in G$.

If the group has a finite number of elements, it is called a **finite group**.

### Operation Notation

For the binary operation, we can use the additive notation or multiplicative notation.

- **Additive**: $a \cdot b = a + b$
- **Multiplicative**: $a \cdot b = ab$

### Examples

- The integers under addition $(\mathbb{Z}, +)$.
- The integers modulo $n$ under addition $(\mathbb{Z}_n, +)$.

## Rings

A ring is a set equipped with two binary operations, addition and multiplication, that satisfy certain axioms. A ring $(R, +, \times)$ consists of the following components:

- A set of elements $R$.
- An addition operation (denoted as $+$) that takes two elements and produces a third element.
- A multiplication operation (denoted as $\times$) that takes two elements and produces a third element.

The operations must satisfy the following properties:

- **Additive + Multiplicative Closure**: For any two elements $a, b \in R$, the result of the addition is also in the ring: $a + b \in R$ and the result of the multiplication is also in the ring: $a \times b \in R$. The ring $R$ is _closed_ under both addition and multiplication.

- **Additive + Multiplicative Associativity**: For any three elements $a, b, c \in R$, the addition and multiplication operations are associative, meaning $(a + b) + c = a + (b + c)$ and $(a \times b) \times c = a \times (b \times c)$. This property ensures that the order of operations does not matter.

- **Additive Identity**: There exists an element $0 \in R$, called the additive identity, such that for any element $a \in R$, the addition $a + 0 = 0 + a = a$. Nothing is said about multiplication yet.

- **Additive Inverse**: For every element $a \in R$, there exists an element $-a \in R$, called the additive inverse of $a$, such that $a + (-a) = (-a) + a = 0$. The inverse of $a$ is denoted as $-a$.

- **Addition Commutativity**: The addition operation is commutative, meaning $a + b = b + a$ for all elements $a, b \in R$.

- **Distributivity**: For any three elements $a, b, c \in R$, the ring satisfies the distributive property, meaning $a \times (b + c) = (a \times b) + (a \times c)$ and $(b + c) \times a = (b \times a) + (c \times a)$.

If the ring has a **multiplicative identity**, i.e., an element $e \in R$ such that $a \times e = e \times a = a$ for all $a \in R$, then the ring is called a _ring with unity_ and that element $e$ is called a **unity**.

If the multiplication is **commutative**, then the ring is called a **commutative ring**.

If the ring has a finite number of elements, it is called a **finite ring**.

### Examples

- The set $\mathbb{Z}$ of all integers, and is a commutative ring with unity.
- The set $\mathbb{Q}$ of all rational numbers.
- The set $\mathbb{R}$ of all real numbers.
- The set $\mathbb{C}$ of all complex numbers.

## Fields

A field is a ring $(F, +, \times)$ with the following properties:

- $F$ is a commutative ring.
- There is a non-zero unity $e \in F$.
- Every non-zero element $a \in F$ have a multiplicative inverse $a^{-1} \in F$ such that $a \times a^{-1} = a^{-1} \times a = e$.

If the field has a finite number of elements, it is called a **finite field**.

### Examples

- The ring of integers modulo $p$, denoted as $\mathbb{Z}_p$, where $p$ is a prime number, is a finite field.
- We can show the example above as $\mathbb{F}_p$ as well, or $\text{GF}(p)$ sometimes for "Galois Field".

[➡️ NEXT SECTION](./3-usage.md)
