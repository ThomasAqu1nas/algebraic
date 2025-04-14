use primitive_types::U256;

use crate::algebra::{elliptic_curve::EllipticCurve, field::FiniteField, group::element::GroupOps, operations::{Additive, Multiplicative}, properties::{Commutative, Distributive, Finite}};

#[derive(PartialEq, Clone, Copy)]
pub struct AffinePoint {
    pub x: U256,
    pub y: U256,
}

#[derive(PartialEq, Clone, Copy)]
pub struct JacobianPoint {
    pub x: U256,
    pub y: U256,
    pub z: U256
}

impl JacobianPoint {
    pub fn infinity() -> Self {
        Self { x: U256::zero(), y: U256::zero(), z: U256::one() }
    }

    pub fn is_infinity(&self) -> bool {
        self.eq(&Self::infinity())
    }
}

impl From<AffinePoint> for JacobianPoint {
    fn from(value: AffinePoint) -> Self {
        Self { x: value.x, y: value.y, z: U256::one() }
    }
}

impl Into<Option<AffinePoint>> for JacobianPoint {
    fn into(self) -> Option<AffinePoint> {
        if self.is_infinity() {
            None
        } else {
            Some()
        }
    }
}



