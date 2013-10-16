use board;
use termbox;

struct Game { board: board::Board, x: int, y: int, player: char }

pub fn new() -> Game {
	let mut game = Game { board: board::new(), x: 0, y: 0, player: 'X' };
	game.draw(); game.moveto(1,1); game
}

impl Game {
	pub fn finished(&self) -> Option<char> { self.board.finished() }
	pub fn play(&mut self) -> bool {
		if !self.board.is_empty(self.x, self.y) { return false; }
		self.board.change(self.x, self.y, self.player);
		self.moveto(self.x, self.y);
		self.player = if self.player == 'X' { 'O' } else { 'X' };
		self.finished().is_none()
	}
	pub fn move(&mut self, ek: u16) {
		if ek == termbox::KEY_ARROW_UP { self.moveto(self.x, self.y-1); }
		if ek == termbox::KEY_ARROW_DOWN { self.moveto(self.x, self.y+1); }
		if ek == termbox::KEY_ARROW_LEFT { self.moveto(self.x-1, self.y); }
		if ek == termbox::KEY_ARROW_RIGHT { self.moveto(self.x+1, self.y); }
	}
	pub fn moveto(&mut self, x: int, y: int) {
		if x < 0 || x > 2 || y < 0 || y > 2 { return; }
		termbox::change_cell(2+self.x*2,2+self.y*2,self.board.get(self.x,self.y),termbox::DEFAULT,termbox::DEFAULT);
		self.x = x; self.y = y;
		termbox::change_cell(2+self.x*2,2+self.y*2,self.board.get(self.x,self.y),termbox::REVERSE,termbox::REVERSE);
		termbox::present();
	}
	pub fn draw(&self) {
		for x in range(1,8) {
			for y in range(1,8) {
				let ch = match (x%2,y%2) {
					(1,1) => '+', (0,0) => ' ', (0,1) => '-', (1,0) => '|', _ => fail!()
				};
				termbox::change_cell(x,y,ch,termbox::DEFAULT,termbox::DEFAULT);
			}
		}
		termbox::present();
	}
	pub fn final(&self) {
		print("\n");
		for y in range(1,8) {
			print(" ");
			for x in range(1,8) {
				print(match (x%2,y%2) {
					(1,1) => '+', (0,0) => self.board.get(x/2-1,y/2-1), (0,1) => '-', (1,0) => '|', _ => fail!()
				}.to_str());
			}
			print("\n");
		}
		match self.finished() {
			Some(ch@'X') | Some(ch@'O') => println!("\n  {} WON!", ch),
			_ => println("\n  TIE!"),
		};
	}
}
