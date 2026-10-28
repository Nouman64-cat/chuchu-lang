#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Array { element: NumericType, len: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericType {
    Int,
    Float,
}

impl NumericType {
    pub fn scalar(self) -> Type {
        match self {
            Self::Int => Type::Int,
            Self::Float => Type::Float,
        }
    }
}

impl Type {
    pub fn is_numeric(self) -> bool {
        matches!(self, Self::Int | Self::Float)
    }
    pub fn accepts(self, source: Self) -> bool {
        self == source || (self == Self::Float && source == Self::Int)
    }
}

pub const MAX_ARRAY_LEN: usize = 256;
