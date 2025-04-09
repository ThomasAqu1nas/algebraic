#![allow(non_camel_case_types)]

// -----------------------------------------------------------------------------
// External and internal module imports
// -----------------------------------------------------------------------------
use modular_math::mod_math::ModMath;
use primitive_types::U256;

use crate::{
    algebra::{
        field::{Field, FiniteField},
        group::{element::GroupOps, Group},
        operations::{Additive, BinaryOperation, Multiplicative},
        properties::{
            Associative, Commutative, Distributive, Finite, Identity, Invertible,
        },
        AlgebraicStructure,
    },
    helpers::mod_inverse,
};

// -----------------------------------------------------------------------------
// Definition of the finite field Fp_7 (Field of integers modulo 7)
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct Fp_7 {
    // The underlying value stored as a U256
    pub value: U256,
}

// -----------------------------------------------------------------------------
// Implement AlgebraicStructure for Fp_7 (both for additive and multiplicative operations)
// -----------------------------------------------------------------------------
impl AlgebraicStructure<Additive> for Fp_7 {
    // Both operations use U256 as the element type
    type Element = U256;
}

impl AlgebraicStructure<Multiplicative> for Fp_7 {
    type Element = U256;
}

// -----------------------------------------------------------------------------
// Implement BinaryOperation for addition on Fp_7 elements
// (Computes (a + b) mod 7)
// -----------------------------------------------------------------------------
impl BinaryOperation<Additive, Fp_7> for U256 {
    fn op(
        a: &<Fp_7 as AlgebraicStructure<Additive>>::Element,
        b: &<Fp_7 as AlgebraicStructure<Additive>>::Element,
    ) -> <Fp_7 as AlgebraicStructure<Additive>>::Element {
        (a + b) % U256::from(7)
    }
}

// -----------------------------------------------------------------------------
// Implement BinaryOperation for multiplication on Fp_7 elements
// (Computes (a * b) mod 7)
// -----------------------------------------------------------------------------
impl BinaryOperation<Multiplicative, Fp_7> for U256 {
    fn op(
        a: &<Fp_7 as AlgebraicStructure<Multiplicative>>::Element,
        b: &<Fp_7 as AlgebraicStructure<Multiplicative>>::Element,
    ) -> <Fp_7 as AlgebraicStructure<Multiplicative>>::Element {
        ((a % U256::from(7)) * (b % U256::from(7))) % U256::from(7)
    }
}

// -----------------------------------------------------------------------------
// Constructor and helper methods for Fp_7
// -----------------------------------------------------------------------------
impl Fp_7 {
    /// Creates a new Fp_7 element by applying modulo reduction to the given value.
    pub fn new(value: U256) -> Self {
        Self {
            // Use ModMath utility to ensure the value is reduced modulo 7
            value: ModMath::new(U256::from(7)).modulus(value),
        }
    }
}

// -----------------------------------------------------------------------------
// Implement the Finite trait for Fp_7 and for U256 (for compatibility)
// -----------------------------------------------------------------------------
impl Finite for Fp_7 {
    /// Returns the order of the finite field Fp_7, which is 7.
    fn order(&self) -> U256 {
        U256::from(7)
    }
}

// This implementation might be used to satisfy trait bounds elsewhere.
impl Finite for U256 {
    /// Here, U256’s order is defined as U256::MAX; this is likely a placeholder.
    fn order(&self) -> U256 {
        U256::MAX
    }
}

// -----------------------------------------------------------------------------
// Field and FiniteField implementations for Fp_7
// -----------------------------------------------------------------------------
impl Field<U256> for Fp_7 {}
impl FiniteField<U256> for Fp_7 {}

// -----------------------------------------------------------------------------
// Distributive, Commutative, Group, and Associative trait implementations
// These implementations are marked for both Fp_7 and U256 as needed.
// -----------------------------------------------------------------------------
impl Distributive<Fp_7, U256> for Fp_7 {}
impl Distributive<Fp_7, U256> for U256 {}

impl Commutative<Additive, Fp_7> for U256 {}
impl Commutative<Multiplicative, Fp_7> for U256 {}

impl Group<Additive> for Fp_7 {}
impl Group<Multiplicative> for Fp_7 {}

impl Associative<Additive, Fp_7> for U256 {}
impl Associative<Multiplicative, Fp_7> for U256 {}

impl GroupOps<Additive, Fp_7> for U256 {}
impl GroupOps<Multiplicative, Fp_7> for U256 {}

// -----------------------------------------------------------------------------
// Implement Invertible trait for finding multiplicative and additive inverses.
// -----------------------------------------------------------------------------
impl Invertible<Multiplicative, Fp_7> for U256 {
    /// Computes the multiplicative inverse of an element a in Fp_7.
    /// Returns None if a is zero.
    fn inverse(
        a: &<Fp_7 as AlgebraicStructure<Multiplicative>>::Element,
    ) -> Option<<Fp_7 as AlgebraicStructure<Multiplicative>>::Element> {
        if a.eq(&U256::zero()) {
            return None;
        }
        // Use the mod_inverse helper to compute the inverse modulo 7
        mod_inverse(*a, U256::from(7))
    }
}

impl Invertible<Additive, Fp_7> for U256 {
    /// Computes the additive inverse of an element a in Fp_7.
    /// Conventionally, the additive inverse is defined as (7 - a) mod 7.
    fn inverse(
        a: &<Fp_7 as AlgebraicStructure<Additive>>::Element, // Prefer using AlgebraicStructure<Additive>
    ) -> Option<<Fp_7 as AlgebraicStructure<Additive>>::Element> {
        Some((U256::from(7) - a) % U256::from(7))
    }
}

// -----------------------------------------------------------------------------
// Implement Identity trait to provide the neutral elements.
// -----------------------------------------------------------------------------
impl Identity<Multiplicative, Fp_7> for Fp_7 {
    /// Returns the multiplicative identity (1) in Fp_7.
    fn identity() -> U256 {
        U256::one()
    }
}

impl Identity<Additive, Fp_7> for Fp_7 {
    /// Returns the additive identity (0) in Fp_7.
    fn identity() -> U256 {
        U256::zero()
    }
}
