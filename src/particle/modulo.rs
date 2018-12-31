pub trait Modulo<RHS = Self> {
    type Output;
    fn modulo(&self, rhs: &RHS) -> Self::Output;
}
