# algebraic

🧮 **Algebraic** is an experimental and educational Rust library for modeling algebraic structures such as groups and fields with strict type safety.  
It is part of my personal journey to deeply understand modern cryptographic schemes and algebraic foundations through practical implementation.

---

## 🚀 Goals

- Learn and internalize the core algebraic structures used in cryptography (groups, fields, etc.)
- Create a minimal but type-safe abstraction of operations like addition and multiplication
- Lay the foundation for implementing cryptographic primitives such as:
  - Finite fields (𝔽ₚ, 𝔽₂ⁿ)
  - Elliptic curves
  - Pairings
  - Zero-knowledge constructions
- Build confidence in applying formal algebra to real cryptographic protocols

---

## 📦 Features

- Abstract binary operations using type markers: `Additive`, `Multiplicative`
- Generic trait-based definitions of:
  - `Group<O>` — parametric over operation
  - `Field` — combines additive and multiplicative groups with the distributive law
- Algebraic properties as traits:
  - `Commutative`, `Associative`, `Invertible`, `Identity`
  - `Distributive<*, +>` — models distributivity of one operation over another
- Explicit semantic distinction between additive and multiplicative field elements using:
  - `AdditiveElement<T>`
  - `MultiplicativeElement<T>`
- Built to enable future proof-checking and algebraic verification at the type level

---

## 🧠 Philosophy

This library is not intended for production use (yet). Instead, it's an experiment in combining:

- Mathematical rigor 📐
- Rust’s type system 🦀
- Cryptographic curiosity 🔐

It is heavily inspired by how cryptography expresses itself through group theory and field arithmetic — and the desire to write code that feels as close as possible to mathematical notation and structure.

---

## 📁 Structure

- `operation.rs` — additive/multiplicative binary operations
- `group.rs` — generic group abstraction
- `field.rs` — trait for fields combining two group operations
- `properties.rs` — algebraic laws as traits
- `wrappers.rs` — `AdditiveElement<T>` and `MultiplicativeElement<T>`

---

## 📚 What's Next?

- [ ] Implement finite fields like 𝔽₂⁵⁶ and 𝔽₂⁵⁵–19
- [ ] Add cyclic group generators and element order computations
- [ ] Implement elliptic curves over fields (Weierstrass & Edwards)
- [ ] Implement basic cryptographic primitives (ECDH, ECDSA, Poseidon hash)
- [ ] Explore zero-knowledge concepts (e.g., R1CS)
- [ ] And much more...

---

## 👋 About Me

I'm a student exploring cryptography, algebra, and systems programming.  
This library is a learning project to strengthen my understanding of modern
programmable cryptography from the ground up.

---
