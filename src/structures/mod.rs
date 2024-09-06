use std::{f64, iter::zip};



#[derive(Debug, Clone)]
struct SquareMatrix {
	data: Vec<Vec<f64>>
}

impl SquareMatrix {
	fn new(n: usize) -> Self {
		SquareMatrix { data: vec![vec![f64::INFINITY; n]; n] }
	}

	fn dim(&self) -> usize {
		self.data.len()
	}

	fn get_cell(&self, i: usize, j: usize) -> Option<f64> {
		if !(self.valid(i) && self.valid(j)) { None }
		else {
			Some(self.data[i][j])
		}
	}

	fn get_cell_mut(&mut self, i: usize, j: usize) -> Option<&mut f64> {
		if !(self.valid(i) && self.valid(j)){ None }
		else {
			Some(&mut self.data[i][j])
		}
	}


	fn valid(&self, i: usize) -> bool {
		i < self.dim()
	}
}


#[derive(Debug, Clone)]
pub struct Grafo {
	cost_matrix: SquareMatrix,
}

impl Grafo {
	pub fn new(n: usize) -> Self {
		Grafo { cost_matrix: SquareMatrix::new(n) }
	}

	pub fn num_vert(&self) -> usize {
		self.cost_matrix.dim()
	}

	pub fn add_edge(&mut self, u: usize, v: usize, cost: f64) {
		if let Some(x) = self.cost_matrix.get_cell_mut(u,v) {
		    *x = cost
		}
	}

	pub fn neighbours(&self, v: usize) -> Vec<(usize, f64)> {
		self.cost_matrix
			.data[v].clone()
			.into_iter()
			.enumerate()
			.filter_map(|(i,x)|{
				if x != f64::INFINITY {
					Some((i, x))
				} else {
					None
				}
			}).collect()
	}
}
