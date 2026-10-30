# Bond Percolation

A Rust library for generating random lattices and analyzing clusters in bond percolation simulations.

## Installation

Add `bond_percolation` to your `Cargo.toml`:

```bash
cargo add bond_percolation
```

To enable lattice visualization, activate the optional `plot` feature:

```bash
cargo add bond_percolation --features plot
```

## Usage

### Generate a random lattice and compute cluster sizes

```rust
use bond_percolation::Lattice;

fn main() {
    let lattice = Lattice::new(30, 0.5);

    println!("{:?}", lattice.get_clusters_size());
    println!("{}", lattice.percolation());
}
```

This example generates a random square lattice of side length 30, where each edge is independently open with probability 0.5. It then identifies connected components using a Union-Find algorithm, retrieves their sizes, and determines whether a percolating cluster exists.

### Visualize a lattice

Visualization requires the optional `plot` feature.

```rust
use bond_percolation::Lattice;
use bond_percolation::plot::plot_lattice;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lattice = Lattice::new_random(30, 0.5);

    plot_lattice(&lattice, "lattice.svg")?;

    Ok(())
}
```

This example generates a random lattice and saves its visualization to `lattice.svg`.
