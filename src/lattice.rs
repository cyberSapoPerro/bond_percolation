use rand::RngExt;

fn vertex_id(row: usize, col: usize, length: usize) -> usize {
    row * length + col
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size:vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);
        if root_a == root_b {
            return;
        }
        if self.size[root_a] < self.size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }
        self.parent[root_b] = root_a;
        self.size[root_a] += self.size[root_b];
    }

    fn get_clusters_data(&self) -> Vec<usize> {
        self.parent
            .iter()
            .enumerate()
            .filter_map(
                |(id, &parent)| {
                    if (id == parent) && (self.size[id] >= 2) 
                    {Some(self.size[id])} else {None}
                }
            )
            .collect()
    }

    fn union_find_algorithm(
        &mut self,
        lattice: (&Vec<Vec<i32>>, &Vec<Vec<i32>>),
        length: usize
    ) {
        let (horizontal_edges, vertical_edges) = lattice;
        for (row, edges) in horizontal_edges.iter().enumerate() {
            for (col, &is_open) in edges.iter().enumerate() {
                if is_open == 1 {
                    let u = vertex_id(row, col, length);
                    let v = vertex_id(row, col + 1, length);
                    self.union(u, v);
                }
            }
        }
        for (row, edges) in vertical_edges.iter().enumerate() {
            for (col, &is_open) in edges.iter().enumerate() {
                if is_open == 1 {
                    let u = vertex_id(row, col, length);
                    let v = vertex_id(row + 1, col, length);
                    self.union(u, v);
                }
            }
        }
    }
}

/// A square lattice for bond percolation.
///
/// The lattice consists of `length × length` vertices connected by
/// horizontal and vertical edges. Each edge is independently opened
/// with the probability specified during construction.
///
/// Inactive edges are labeled -1. Labels greater than or equal to 1
/// identify connected clusters.
pub struct Lattice {
    length: usize,
    horizontal_edges: Vec<Vec<i32>>,
    vertical_edges: Vec<Vec<i32>>,
    union_find: UnionFind,
}

#[derive(Debug)]
pub enum LatticeError {
    ShortLengthSize,
    ParameterOutOfBounds,
}

impl std::fmt::Display for LatticeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ShortLengthSize => {
                write!(f, "Length should be greatter than 2")
            }
            Self::ParameterOutOfBounds => {
                write!(f, "The parameter should be between 0 and 1")
            }
        }
    }
}

impl std::error::Error for LatticeError {}

impl Lattice {
    /// Returns the side length of the square lattice, measured in vertices.
    pub fn length(&self) -> usize {
        self.length
    }

    /// Returns references to the horizontal and vertical edge matrices.
    ///
    /// The returned tuple contains:
    ///
    /// - The horizontal edge matrix, with dimensions
    ///   `length × (length - 1)`.
    /// - The vertical edge matrix, with dimensions
    ///   `(length - 1) × length`.
    ///
    /// Every open edge contains the root vertex identifier of its connected component.
    /// Closed edges remain represented by `-1`.
    ///
    /// The returned references provide read-only access to the matrices.
    pub fn lattice(&self) -> (&[Vec<i32>], &[Vec<i32>]) {
        (&self.horizontal_edges, &self.vertical_edges)
    }

    /// Creates a random square lattice for bond percolation.
    ///
    /// Each horizontal and vertical edge is independently opened with
    /// probability `parameter` and closed with probability `1 - parameter`.
    ///
    /// After generating the edges, the method identifies connected
    /// components using the union-find algorithm and labels every open edge
    /// with the root vertex identifier of the component containing its
    /// first endpoint.
    ///
    /// # Arguments
    ///
    /// * `length` - Number of vertices along each side of the lattice.
    /// * `parameter` - Probability of an edge being open, in the interval
    ///   `[0, 1]`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the constructed lattice on success, or a
    /// `LatticeError` if an argument is invalid.
    ///
    /// # Errors
    ///
    /// Returns [`LatticeError::ShortLengthSize`] if `length < 2`.
    ///
    /// Returns [`LatticeError::ParameterOutOfBounds`] if `parameter`
    /// is outside `[0, 1]` or is NaN.
    pub fn new(length: usize, parameter: f64) -> Result<Self, LatticeError> {

        if length < 2 {
            return Err(LatticeError::ShortLengthSize)
        }
        if !(0.0..=1.0).contains(&parameter) {
            return Err(LatticeError::ParameterOutOfBounds)
        }

        let (mut horizontal_edges, mut vertical_edges) =
            Lattice::generate_random_edges(length, parameter);
        let mut uf = UnionFind::new(length*length);

        uf.union_find_algorithm(
            (&horizontal_edges, &vertical_edges),
            length
            );

        Lattice::label_lattice(
            (&mut horizontal_edges, &mut vertical_edges), 
            length, 
            &mut uf
            );


        Ok(Self {
            length,
            horizontal_edges,
            vertical_edges,
            union_find: uf,
        })
    }

    fn generate_random_edges(
        length: usize,
        parameter: f64
    ) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
        let mut rng = rand::rng();

        let mut horizontal_edges = vec![vec![0; length - 1]; length];
        let mut vertical_edges = vec![vec![0; length]; length - 1];

        for row in &mut horizontal_edges {
            for edge in row.iter_mut() {
                *edge = if rng.random_bool(parameter) {1} else {-1}
            }
        }
        for row in &mut vertical_edges {
            for edge in row.iter_mut() {
                *edge = if rng.random_bool(parameter) {1} else {-1}
            }
        }
        (horizontal_edges, vertical_edges)
    }

    fn label_lattice(
        lattice: (&mut Vec<Vec<i32>>, &mut Vec<Vec<i32>>),
        length: usize,
        uf: &mut UnionFind,
        ){
        let (horizontal_edges, vertical_edges) = lattice;
        for row in 0..length {
            for col in 0..length - 1 {
                if horizontal_edges[row][col] == 1 {
                    let u = vertex_id(row, col, length);
                    horizontal_edges[row][col] = uf.find(u) as i32;
                }
            }
        }
        for row in 0..length - 1 {
            for col in 0..length {
                if vertical_edges[row][col] == 1 {
                    let u = vertex_id(row, col, length);
                    vertical_edges[row][col] = uf.find(u) as i32;
                }
            }
        }
    }

    /// Returns the sizes of all non-singleton connected clusters.
    ///
    /// Each element of the returned vector is the number of vertices in a
    /// distinct cluster containing at least two vertices. Isolated vertices
    /// are excluded. The vector's order is not guaranteed.
    pub fn get_clusters_size(&self) -> Vec<usize> {
        self.union_find.get_clusters_data()
    }

    /// Returns true if there is a cluster that goes from the top to the bottom of the lattice.
    pub fn percolation(&self) -> bool {
        let top = &self.horizontal_edges[0];
        let bottom = &self.horizontal_edges[self.length - 1];

        top.iter()
            .any(
                |&cluster|
                cluster != -1 && bottom.contains(&cluster)
                )
    }
}
