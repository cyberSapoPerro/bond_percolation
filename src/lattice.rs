use rand::RngExt;

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
}

/// A square lattice for bond percolation.
///
/// The lattice consists of `length × length` vertices connected by
/// horizontal and vertical edges. Each edge is independently opened
/// according to the specified probability during construction.
///
/// Open edges are represented by `1` and closed edges by `-1`.
/// After [`union_find_algorithm`](RandomLatice::union_find_algorithm)
/// is called, open edges are labeled with the identifier of the
/// connected cluster containing their first endpoint.
///
/// The structure also provides methods to retrieve the lattice dimensions
/// and edge configurations, and to obtain the sizes of connected clusters.
pub struct RandomLatice {
    length: usize,
    horizontal_edges: Vec<Vec<i32>>,
    vertical_edges: Vec<Vec<i32>>,
    union_find: UnionFind,
}

#[derive(Debug)]
pub enum RandomLaticeError {
    ShortLengthSize,
    ParameterOutOfBounds,
}

impl std::fmt::Display for RandomLaticeError {
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

impl std::error::Error for RandomLaticeError {}

impl RandomLatice {
    /// Returns the side length of the square lattice, measured in vertices.
    pub fn length(&self) -> usize {
        self.length
    }

    /// Returns the horizontal edges of the lattice.
    ///
    /// Each row contains `length - 1` edges. An edge is represented by `1`
    /// if it is open, or `-1` if it is closed. After calling
    /// [`union_find_algorithm`](Self::union_find_algorithm), open edges
    /// contain the label of their connected cluster instead.
    pub fn horizontal_edges(&self) -> &[Vec<i32>] {
        &self.horizontal_edges
    }

    /// Returns the vertical edges of the lattice.
    ///
    /// Each row contains `length` edges. An edge is represented by `1`
    /// if it is open, or `-1` if it is closed. After calling
    /// [`union_find_algorithm`](Self::union_find_algorithm), open edges
    /// contain the label of their connected cluster instead.
    pub fn vertical_edges(&self) -> &[Vec<i32>] {
        &self.vertical_edges
    }

    /// Creates a square lattice for bond percolation.
    ///
    /// Each edge is independently opened with probability `parameter`
    /// and closed with probability `1 - parameter`.
    ///
    /// # Arguments
    ///
    /// * `length` - Number of vertices along each side of the lattice.
    /// * `parameter` - Probability of an edge being open, in the interval
    ///   `[0, 1]`.
    ///
    /// # Errors
    ///
    /// Returns [`RandomLaticeError::ShortLengthSize`] if `length` is less
    /// than `2`, or [`RandomLaticeError::ParameterOutOfBounds`] if
    /// `parameter` is outside `[0, 1]` or is NaN.
    pub fn new(length: usize, parameter: f64) -> Result<Self, RandomLaticeError> {

        if length < 2 {
            return Err(RandomLaticeError::ShortLengthSize)
        }
        if !(0.0..=1.0).contains(&parameter) {
            return Err(RandomLaticeError::ParameterOutOfBounds)
        }

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
        Ok(Self {
            length,
            horizontal_edges,
            vertical_edges,
            union_find: UnionFind::new(length*length),
        })
    }

    fn vertex_id(&self, row: usize, col: usize) -> usize {
        row * self.length + col
    }

    /// Applies the Union-Find algorithm to identify connected clusters.
    ///
    /// Vertices connected by open edges are merged into the same cluster.
    /// The method then labels each open edge with the identifier of the
    /// cluster containing its first endpoint. Closed edges remain unchanged.
    ///
    /// Calling this method modifies the edge arrays. The labels are internal
    /// vertex identifiers and are not guaranteed to be consecutive cluster
    /// numbers.
    pub fn union_find_algorithm(&mut self) {
        // Union Find Algortihm
        for (row, edges) in self.horizontal_edges.iter().enumerate() {
            for (col, &is_open) in edges.iter().enumerate() {
                if is_open == 1 {
                    let u = self.vertex_id(row, col);
                    let v = self.vertex_id(row, col + 1);
                    self.union_find.union(u, v);
                }
            }
        }
        for (row, edges) in self.vertical_edges.iter().enumerate() {
            for (col, &is_open) in edges.iter().enumerate() {
                if is_open == 1 {
                    let u = self.vertex_id(row, col);
                    let v = self.vertex_id(row + 1, col);
                    self.union_find.union(u, v);
                }
            }
        }

        // Labeling
        for row in 0..self.length {
            for col in 0..self.length - 1 {
                if self.horizontal_edges[row][col] == 1 {
                    let u = self.vertex_id(row, col);
                    self.horizontal_edges[row][col] = self.union_find.find(u) as i32;
                }
            }
        }
        for row in 0..self.length - 1 {
            for col in 0..self.length {
                if self.vertical_edges[row][col] == 1 {
                    let u = self.vertex_id(row, col);
                    self.vertical_edges[row][col] = self.union_find.find(u) as i32;
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
}
