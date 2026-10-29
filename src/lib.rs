mod lattice;

pub use lattice::Lattice;

#[cfg(feature = "plot")]
pub mod plot;

#[cfg(test)]
mod tests;
