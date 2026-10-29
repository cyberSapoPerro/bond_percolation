mod lattice;

pub use lattice::Lattice;
pub use lattice::LatticeError;

#[cfg(feature = "plot")]
pub mod plot;

#[cfg(test)]
mod tests;
