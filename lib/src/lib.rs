pub mod nat;
mod private {
    pub trait Sealed {}
}
#[cfg(test)]
mod test;