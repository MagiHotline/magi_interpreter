use std::{fmt::Display, ops::{Add, BitAnd, BitOr, Div, Mul, Not, Rem, Sub}, panic};

/// Struct that implements type of values
#[derive(Debug, Clone, Copy, PartialOrd)]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    Void
}

impl Default for Value {
    fn default() -> Self {
        Value::Void
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(int) => write!(f, "{}", int),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Void => panic!("Cannot print void values"),
        }
    }
}


impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(left), Value::Int(right)) => Value::Int(left + right),
            (Value::Float(left), Value::Float(right)) => Value::Float(left + right),
            _ => panic!("Type Mismatch: cannot apply add")
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(left), Value::Int(right)) => Value::Int(left - right),
            (Value::Float(left), Value::Float(right)) => Value::Float(left - right),
            _ => panic!("Type Mismatch: cannot apply sub")
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(left), Value::Int(right)) => Value::Int(left * right),
            (Value::Float(left), Value::Float(right)) => Value::Float(left * right),
            _ => panic!("Type Mismatch: cannot apply mul")
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(left), Value::Int(right)) => Value::Int(left / right),
            (Value::Float(left), Value::Float(right)) => Value::Float(left / right),
            _ => panic!("Type Mismatch: cannot apply div")
        }
    }
}

impl Rem for Value {
    type Output = Value;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(left), Value::Int(right)) => Value::Int(left % right),
            (Value::Float(left), Value::Float(right)) => Value::Float(left % right),
            _ => panic!("Type Mismatch: cannot apply modulo operator")
        }
    }
}

impl BitAnd for Value {
    type Output = Value;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Bool(left), Value::Bool(right)) => Value::Bool(left & right),
            _ => panic!("Type Mismatch: cannot apply bit and")
        }
    }
}

impl BitOr for Value {
    type Output = Value;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Bool(left), Value::Bool(right)) => Value::Bool(left | right),
            _ => panic!("Type Mismatch: cannot apply bit or")
        }
    }
}


impl Not for Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        match self {
            Value::Bool(val) => Value::Bool(!val),
            _ => panic!("Type Mismatch: cannot apply not operator")
        }
    }
}


impl Value {
    pub fn pow(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Int(left), Value::Int(right)) => Value::Int(left.pow(right as u32)),
            (Value::Float(left), Value::Float(right)) => Value::Float(left.powf(right)),
            _ => panic!("Type Mismatch: cannot apply pow operator")
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            _ => panic!("Type Mismatch: cannot apply not eq operator"),
        }
    }
}
