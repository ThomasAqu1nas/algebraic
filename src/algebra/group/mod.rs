use std::ops::Add;

use element::{ElementFinite, GroupOps};
use primitive_types::U256;

use crate::helpers::{gcd, U256Range};

use super::{operations::{Additive, BinaryOperation, BinaryOperationType}, properties::{Commutative, Finite, Identity}};

pub mod element;
pub mod extension;

pub trait Group<O>: 
    std::fmt::Debug
    + Sized
    + Identity<Self::Element>
where O:
    BinaryOperationType<Self::Element>
{
    type Element: GroupOps<O, Self>;
}

pub trait FiniteGroup<O>: Group<O> + Finite 
    where Self::Element: GroupOps<O, Self> + ElementFinite<O, Self>,
    O: BinaryOperationType<Self::Element>
{
    fn element_order(&self, e: &Self::Element) -> U256
    where
        Self::Element: ElementFinite<O, Self>,
    {
        <Self::Element as ElementFinite<O, Self>>::element_order(self, e)
    }
}

pub trait AbelianGroup: Group<Additive>
where 
    Self::Element: GroupOps<Additive, Self> + Commutative<Self::Element>,
    <Self as Group<Additive>>::Element: Add<Output = <Self as Group<Additive>>::Element>,
    for<'a> &'a <Self as Group<Additive>>::Element: Add<Output = <Self as Group<Additive>>::Element>
{}

pub trait CyclingGroup: FiniteGroup<Additive> + AbelianGroup
where 
    Self::Element: GroupOps<Additive, Self> + ElementFinite<Additive, Self> + Commutative<Self::Element>,
    <Self as Group<Additive>>::Element: Add<Output = <Self as Group<Additive>>::Element>,
    for<'a> &'a <Self as Group<Additive>>::Element: Add<Output = <Self as Group<Additive>>::Element>
{
    fn generator(&self) -> Self::Element;

    fn all_generators(&self) -> Vec<Self::Element> {
        let generator = self.generator();
        let order = self.order();

        let mut result = vec![];

        for k in U256Range::new(U256::one(), order) {
            if gcd(k, order) == U256::one() {
                let mut acc = Self::identity();
                for _ in U256Range::new(U256::zero(), k) {
                    acc = <Self::Element as BinaryOperation<Self::Element>>::op(&acc, &generator); 
                }
                result.push(acc);
            }
        }
        result
    }

    fn is_generator(&self, elem: &Self::Element) -> bool {
        let order = self.order();
        <Self::Element as ElementFinite<Additive, Self>>::element_order(self, elem) == order
    }
}
