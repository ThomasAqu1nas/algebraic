use point::JacobianPoint;
use primitive_types::U256;
use crate::algebra::{field::fp_impls::fp_secp256k1::FP_SECP265K1, group::{CyclingGroup, FiniteGroup, Group}, operations::Additive, properties::Finite, AlgebraicStructure};
use super::EllipticCurve;

pub mod point;

pub struct Secp256k1;


impl AlgebraicStructure<Additive> for Secp256k1 {
    type Element = JacobianPoint;
}

impl Group<Additive> for Secp256k1 {

}

impl Finite for Secp256k1 {
    fn order(&self) -> U256 {
        U256::from_str_radix(
            "0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 
            16
        ).unwrap()
    }
}

impl FiniteGroup<Additive> for Secp256k1 {}

impl CyclingGroup<Additive> for Secp256k1 {
    fn generator(&self) -> Self::Element {
        
    }
}

impl EllipticCurve<U256, FP_SECP265K1> for Secp256k1 {
    fn a(&self) -> U256 {
        U256::zero()
    }

    fn b(&self) -> U256 {
        U256::from(7)
    }
}