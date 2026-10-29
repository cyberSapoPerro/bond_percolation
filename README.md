# Bond Percolation

A Rust library for generating random lattices and analyzing clusters in bond percolation simulations.

## Installation

Add `bond_percolation` to your `Cargo.toml`:

```toml
[dependencies]
bond_percolation = "0.1"
```

To enable lattice visualization, activate the optional `plot` feature:

```toml
[dependencies]
bond_percolation = { version = "0.1", features = ["plot"] }
```

## Usage

### Generate a random lattice and compute cluster sizes

```rust
use bond_percolation::RandomLattice;

fn main() {
    let mut lattice = RandomLattice::new(30, 0.5);

    lattice.union_find_algorithm();
    let sizes = lattice.get_clusters_size();

    println!("{:?}", sizes);
}
```

This example generates a random square lattice of side length 30, with each edge independently open with probability 0.5. It then identifies connected clusters using a Union-Find algorithm and retrieves their sizes.

### Visualize a lattice

Visualization requires the optional `plot` feature.

```rust
use bond_percolation::RandomLattice;
use bond_percolation::plot::plot_lattice;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lattice = RandomLattice::new_random(30, 0.5);

    plot_lattice(&lattice, "lattice.svg")?;

    Ok(())
}
```

This example generates a random lattice and saves its visualization to `lattice.svg`.
