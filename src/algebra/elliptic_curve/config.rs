use primitive_types::U256;


pub enum EllipticCurveConfigOption {
    Secp256k1, Ed25119
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EllipticCurveConfig {
    a: U256,
    b: U256,
    p: U256
}