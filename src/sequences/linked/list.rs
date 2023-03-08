// rust

struct Node<D, P, N> {
	data: D,
	next: N,
	prev: P
}

impl<D, P, N> Node<D, P, N> {
	fn last(&self) -> P {
		self.prev
	}

	fn next(&self) -> N {
		self.next
	}

	fn what(&self) -> D {
		self.data
	}

	fn whole(&self) -> (P, D, N) {
		(self.prev, self.data, self.next)
	}
}

fn main() {}