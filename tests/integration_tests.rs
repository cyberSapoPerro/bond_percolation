use bond_percolation::Lattice;

#[test]
fn test_public_api() {
    let lattice = Lattice::new(10, 0.5).unwrap();

    assert_eq!(lattice.length(), 10);

    let (horizontal, vertical) = lattice.lattice();

    assert_eq!(horizontal.len(), 10);
    assert_eq!(vertical.len(), 9);

    let clusters = lattice.get_clusters_size();

    assert!(clusters.iter().all(|&size| size >= 2));
}
