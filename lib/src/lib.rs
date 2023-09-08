pub mod nat;
pub mod logic;
pub mod hlist;
mod private {
    pub trait Sealed {}
}
#[cfg(test)]
mod test;