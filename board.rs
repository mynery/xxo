struct Board { cells: [char, ..9] }

pub fn new() -> Board { Board::new() }

impl Clone for Board {
	fn clone(&self) -> Board {
		let mut clone = [' ', ..9];
		for i in range(0,9) { clone[i] = self.cells[i] }
		Board { cells: clone }
	}
}

impl Board {
	pub fn new() -> Board { Board { cells: [' ', ..9] } }
	pub fn is_empty(&self, x: int, y: int) -> bool { self.get(x, y) == ' ' }
	pub fn get(&self, x: int, y: int) -> char { self.cells[y*3+x] }
	pub fn change<'a>(&'a mut self, x: int, y: int, ch: char) -> &'a mut Board { self.cells[y*3+x] = ch; self }
	pub fn finished(&self) -> Option<char> {
		let size = 3;
		let mut i = -1;
		while i < size - 1 {
			i += 1;
			let rv = self.cells[i*size];
			let cv = self.cells[i];
			let mut c = -1;
			let mut row = true;
			let mut col = true;
			let mut d1 = true;
			let mut d2 = true;
			while c < size - 1 {
				c += 1;
				d1 = d1 && i == 0 && self.cells[c*size+c] == self.cells[0];
				d2 = d2 && i == 0 && self.cells[c*size+size-c-1] == self.cells[size-1];
				row = row && self.cells[i*size+c] == rv;
				col = col && self.cells[i+c*size] == cv;
			}
			if d1 && self.cells[0] != ' ' { return Some(self.cells[0]); }
			if d2 && self.cells[size-1] != ' ' { return Some(self.cells[size-1]); }
			if row && rv != ' ' { return Some(rv); }
			if col && cv != ' ' { return Some(cv); }
		}
		if self.cells.iter().find(|& &x| x == ' ').is_none() { return Some(' '); }
		None
	}
}
