// rust

struct Pair<D> {
	index: i32,
	data: D
}

impl<D> Pair<D> {
	fn num(&self) -> i32 {
		self.index
	}

	fn what(&self) -> D {
		self.data
	}

	fn whole(&self) -> (i32, D) {
		(self.index, self.data)
	}
}

struct Arr<i32, Pair> {
	length: i32
}

impl Arr<T> {
	fn start(&mut self, other: T) {
		// self.head = Pair{}
	}

	fn append(&mut self, other: ) {
		// self.
	}
}
