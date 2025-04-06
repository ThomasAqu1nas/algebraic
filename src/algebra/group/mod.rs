use element::{ElementFinite, GroupOps};
use primitive_types::U256;

use crate::helpers::{gcd, U256Range};

use super::{operations::{Additive, BinaryOperation, BinaryOperationType}, properties::{Commutative, Finite, Identity}, AlgebraicStructure};

pub mod element;
pub mod extension;
pub mod impls;

pub trait Group<O: BinaryOperationType>: 
    std::fmt::Debug
    + Sized
    + AlgebraicStructure
    + Identity<Self>
where 
    Self::Element: GroupOps<O, Self>
{}

pub trait FiniteGroup<O: BinaryOperationType>: Group<O> + Finite 
    where Self::Element: GroupOps<O, Self> + ElementFinite<O, Self>,
{
    fn element_order(&self, e: &Self::Element) -> U256
    where
        Self::Element: ElementFinite<O, Self>,
    {
        <Self::Element as ElementFinite<O, Self>>::element_order(self, e)
    }
}

pub trait AbelianGroup<O: BinaryOperationType>: Group<O>
where 
    Self::Element: GroupOps<O, Self> + Commutative<Self>,
{}

pub trait CyclingGroup<O: BinaryOperationType>: FiniteGroup<O> + AbelianGroup<O>
where 
    Self::Element: GroupOps<O, Self> + ElementFinite<O, Self> + Commutative<Self>,
{
    fn generator(&self) -> Self::Element;

    fn all_generators(&self) -> Vec<Self::Element> {
        let generator = self.generator();
        let order = self.order();

        let mut result = vec![];

        for k in U256Range::new(U256::one(), order) {
            if gcd(k, order) == U256::one() {
                let mut acc: Self::Element = Self::identity();
                for _ in U256Range::new(U256::zero(), k) {
                    acc = Self::Element::op(&acc, &generator); 
                }
                result.push(acc);
            }
        }
        result
    }

    fn is_generator(&self, elem: &Self::Element) -> bool {
        let order = self.order();
        <Self::Element as ElementFinite<O, Self>>::element_order(self, elem) == order
    }
}
