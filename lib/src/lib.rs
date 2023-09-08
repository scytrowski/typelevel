pub mod nat;
pub mod logic;
mod private {
    pub trait Sealed {}
}
#[cfg(test)]
mod test;