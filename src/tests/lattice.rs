use crate::lattice::Lattice;
use crate::lattice::LatticeError;

#[test]
fn test_lattice_length() {
    let lattice = Lattice::new(5, 0.5).unwrap();

    assert_eq!(lattice.length(), 5);
}

#[test]
fn test_invalid_length() {
    let result = Lattice::new(1, 0.5);

    assert!(matches!(
            result,
            Err(LatticeError::ShortLengthSize)
    ));
}

#[test]
fn test_invalid_parameter() {
    assert!(matches!(
            Lattice::new(5, -0.1),
            Err(LatticeError::ParameterOutOfBounds)
    ));

    assert!(matches!(
            Lattice::new(5, 1.1),
            Err(LatticeError::ParameterOutOfBounds)
    ));

    assert!(matches!(
            Lattice::new(5, f64::NAN),
            Err(LatticeError::ParameterOutOfBounds)
    ));
}

#[test]
fn test_edge_dimensions() {
    let length = 5;
    let lattice = Lattice::new(length, 0.5).unwrap();

    let (horizontal, vertical) = lattice.lattice();

    assert_eq!(horizontal.len(), length);
    assert_eq!(vertical.len(), length - 1);

    for row in horizontal {
        assert_eq!(row.len(), length - 1);
    }

    for row in vertical {
        assert_eq!(row.len(), length);
    }
}

#[test]
fn test_zero_probability() {
    let lattice = Lattice::new(5, 0.0).unwrap();
    let (horizontal, vertical) = lattice.lattice();

    assert!(
        horizontal
        .iter()
        .flatten()
        .all(|&edge| edge == -1)
    );

    assert!(
        vertical
        .iter()
        .flatten()
        .all(|&edge| edge == -1)
    );
}

#[test]
fn test_unit_probability() {
    let lattice = Lattice::new(5, 1.0).unwrap();
    let (horizontal, vertical) = lattice.lattice();

    assert!(
        horizontal
        .iter()
        .flatten()
        .all(|&edge| edge != -1)
    );

    assert!(
        vertical
        .iter()
        .flatten()
        .all(|&edge| edge != -1)
    );
}

#[test]
fn test_no_clusters_when_all_edges_are_closed() {
    let lattice = Lattice::new(5, 0.0).unwrap();

    assert!(lattice.get_clusters_size().is_empty());
}

#[test]
fn test_single_cluster_when_all_edges_are_open() {
    let length = 5;
    let lattice = Lattice::new(length, 1.0).unwrap();

    let clusters = lattice.get_clusters_size();
    println!("Cluster sizes: {clusters:?}");

    assert_eq!(
        clusters,
        vec![length * length]
    );
}

#[test]
fn test_no_percolation_when_all_edges_are_closed() {
    let lattice = Lattice::new(5, 0.0).unwrap();

    assert!(!lattice.percolation());
}

#[test]
fn test_percolation_when_all_edges_are_open() {
    let lattice = Lattice::new(5, 1.0).unwrap();

    assert!(lattice.percolation());
}
