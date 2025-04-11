#![allow(non_camel_case_types)]

// -----------------------------------------------------------------------------
// External and internal module imports
// -----------------------------------------------------------------------------
use modular_math::mod_math::ModMath;
use primitive_types::U256;

pub const P: U256 = U256([
    0xFFFFFC2F,
    0xFFFFFFFEFFFFFFFF,
    0xFFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFFF,
]);

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
// Definition of the finite field FP_SECP265K1 (Field of integers modulo 7)
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct FP_SECP265K1 {
    // The underlying value stored as a U256
    pub value: U256,
}

// -----------------------------------------------------------------------------
// Implement AlgebraicStructure for FP_SECP265K1 (both for additive and multiplicative operations)
// -----------------------------------------------------------------------------
impl AlgebraicStructure<Additive> for FP_SECP265K1 {
    // Both operations use U256 as the element type
    type Element = U256;
}

impl AlgebraicStructure<Multiplicative> for FP_SECP265K1 {
    type Element = U256;
}

// -----------------------------------------------------------------------------
// Implement BinaryOperation for addition on FP_SECP265K1 elements
// (Computes (a + b) mod 7)
// -----------------------------------------------------------------------------
impl BinaryOperation<Additive, FP_SECP265K1> for U256 {
    fn op(
        a: &<FP_SECP265K1 as AlgebraicStructure<Additive>>::Element,
        b: &<FP_SECP265K1 as AlgebraicStructure<Additive>>::Element,
    ) -> <FP_SECP265K1 as AlgebraicStructure<Additive>>::Element {
        (a + b) % P
    }
}

// -----------------------------------------------------------------------------
// Implement BinaryOperation for multiplication on FP_SECP265K1 elements
// (Computes (a * b) mod 7)
// -----------------------------------------------------------------------------
impl BinaryOperation<Multiplicative, FP_SECP265K1> for U256 {
    fn op(
        a: &<FP_SECP265K1 as AlgebraicStructure<Multiplicative>>::Element,
        b: &<FP_SECP265K1 as AlgebraicStructure<Multiplicative>>::Element,
    ) -> <FP_SECP265K1 as AlgebraicStructure<Multiplicative>>::Element {
        ((a % P) * (b % P)) % P
    }
}

// -----------------------------------------------------------------------------
// Constructor and helper methods for FP_SECP265K1
// -----------------------------------------------------------------------------
impl FP_SECP265K1 {
    /// Creates a new FP_SECP265K1 element by applying modulo reduction to the given value.
    pub fn new(value: U256) -> Self {
        Self {
            // Use ModMath utility to ensure the value is reduced modulo 7
            value: ModMath::new(P).modulus(value),
        }
    }
}

// -----------------------------------------------------------------------------
// Implement the Finite trait for FP_SECP265K1 and for U256 (for compatibility)
// -----------------------------------------------------------------------------
impl Finite for FP_SECP265K1 {
    /// Returns the order of the finite field FP_SECP265K1, which is 7.
    fn order(&self) -> U256 {
        P
    }
}

// -----------------------------------------------------------------------------
// Field and FiniteField implementations for FP_SECP265K1
// -----------------------------------------------------------------------------
impl Field<U256> for FP_SECP265K1 {}
impl FiniteField<U256> for FP_SECP265K1 {}

// -----------------------------------------------------------------------------
// Distributive, Commutative, Group, and Associative trait implementations
// These implementations are marked for both FP_SECP265K1 and U256 as needed.
// -----------------------------------------------------------------------------
impl Distributive<FP_SECP265K1, U256> for FP_SECP265K1 {}
impl Distributive<FP_SECP265K1, U256> for U256 {}

impl Commutative<Additive, FP_SECP265K1> for U256 {}
impl Commutative<Multiplicative, FP_SECP265K1> for U256 {}

impl Group<Additive> for FP_SECP265K1 {}
impl Group<Multiplicative> for FP_SECP265K1 {}

impl Associative<Additive, FP_SECP265K1> for U256 {}
impl Associative<Multiplicative, FP_SECP265K1> for U256 {}

impl GroupOps<Additive, FP_SECP265K1> for U256 {}
impl GroupOps<Multiplicative, FP_SECP265K1> for U256 {}

// -----------------------------------------------------------------------------
// Implement Invertible trait for finding multiplicative and additive inverses.
// -----------------------------------------------------------------------------
impl Invertible<Multiplicative, FP_SECP265K1> for U256 {
    /// Computes the multiplicative inverse of an element a in FP_SECP265K1.
    /// Returns None if a is zero.
    fn inverse(
        a: &<FP_SECP265K1 as AlgebraicStructure<Multiplicative>>::Element,
    ) -> Option<<FP_SECP265K1 as AlgebraicStructure<Multiplicative>>::Element> {
        if a.eq(&U256::zero()) {
            return None;
        }
        // Use the mod_inverse helper to compute the inverse modulo 7
        mod_inverse(*a, P)
    }
}

impl Invertible<Additive, FP_SECP265K1> for U256 {
    /// Computes the additive inverse of an element a in FP_SECP265K1.
    /// Conventionally, the additive inverse is defined as (7 - a) mod 7.
    fn inverse(
        a: &<FP_SECP265K1 as AlgebraicStructure<Additive>>::Element, // Prefer using AlgebraicStructure<Additive>
    ) -> Option<<FP_SECP265K1 as AlgebraicStructure<Additive>>::Element> {
        Some((P - a) % P)
    }
}

// -----------------------------------------------------------------------------
// Implement Identity trait to provide the neutral elements.
// -----------------------------------------------------------------------------
impl Identity<Multiplicative, FP_SECP265K1> for FP_SECP265K1 {
    /// Returns the multiplicative identity (1) in FP_SECP265K1.
    fn identity() -> U256 {
        U256::one()
    }
}

impl Identity<Additive, FP_SECP265K1> for FP_SECP265K1 {
    /// Returns the additive identity (0) in FP_SECP265K1.
    fn identity() -> U256 {
        U256::zero()
    }
}
