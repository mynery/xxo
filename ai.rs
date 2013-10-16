use board;
use game;

pub enum GameTree { Win(int, int, char), Tie(int, int), State(int, int, int, ~[~GameTree]) }
struct AI { player: char, tree: GameTree }

pub fn new(player: char, opponent: char) -> AI { AI::new(player, opponent) }

trait GameTreeList {
	fn rate(&mut self, player: char, max: bool) -> int;
	fn follow(&mut self, x: int, y: int) -> Option<GameTree>;
	fn get_move(&self, player: char) -> (int, int);
}

impl GameTree {
	pub fn rate(&mut self, player: char, max: bool) {
		match *self {
			State(_, _, ref mut rating, ref mut trees) => { *rating = trees.rate(player, max); }, _ => ()
		};
	}
	pub fn follow(&mut self, x: int, y: int) -> Option<GameTree> {
		match *self { State(_, _, _, ref mut trees) => trees.follow(x, y), _ => None }
	}
	pub fn get_move(&self, player: char) -> (int, int) {
		match *self { State(_, _, _, ref trees) => trees.get_move(player), _ => (-1, -1) }
	}
	pub fn generate_with(board: &board::Board, ax: int, ay: int, player: char, opponent: char) -> GameTree {
		match board.finished() {
			Some(' ') => Tie(ax, ay), Some(ch) => Win(ax, ay, ch),
			None => {
				let mut trees: ~[~GameTree] = ~[];
				for x in range(0, 3) {
					for y in range(0, 3) {
						if board.is_empty(x, y) {
							let mut nboard = board.clone();
							nboard.change(x, y, player);
							trees.push(~GameTree::generate_with(&nboard, x, y, opponent, player));
						}
					}
				}
				State(ax, ay, 0, trees)
			}
		}
	}
	pub fn generate(player: char, opponent: char) -> GameTree {
		let mut tree = GameTree::generate_with(&mut board::new(), -1, -1, opponent, player);
		tree.rate(player, player == 'X');
		tree
	}
}

impl GameTreeList for ~[~GameTree] {
	fn rate(&mut self, player: char, max: bool) -> int {
		let mut res = if max { 0 } else { 100 };
		let mut t: int;
		for tree in self.mut_iter() {
			tree.rate(player, !max);
			t = match **tree {
				Win(_, _, ch) => { if ch == player { 100 } else { 0 } },
				State(_, _, rating, _) => rating, Tie(_, _) => 50
			};
			res = if max { if res > t { res } else { t } } else { if res > t { t } else { res } }
		}
		res
	}
	fn follow(&mut self, x: int, y: int) -> Option<GameTree> {
		let mut rindex = -1;
		for (index, tree) in self.iter().enumerate() {
			match **tree {
				State(nx, ny, _, _) | Win(nx, ny, _) | Tie(nx, ny) if (nx, ny) == (x, y) => rindex = index, _ => ()
			}
		}
		if rindex < 0 || rindex > self.len() {
			None
		} else {
			Some(*(self.remove(rindex)))
		}
	}
	fn get_move(&self, player: char) -> (int, int) {
		let mut rx = -1; let mut ry = -1;
		let mut maxrate = 0;
		for tree in self.iter() {
			maxrate = match **tree {
				State(x, y, rate, _) if rate > maxrate => { rx = x; ry = y; rate },
				Win(x, y, ch) if ch == player => { rx = x; ry = y; 100 },
				Tie(x, y) if maxrate < 50 => { rx = x; ry = y; 50 }, _ => maxrate
			}
		}
		(rx, ry)
	}
}

impl AI {
	pub fn new(player: char, opponent: char) -> AI {
		AI { player: player, tree: GameTree::generate(player, opponent) }
	}
	pub fn move(&mut self, game: &mut game::Game) {
		if !game.board.cells.iter().find(|& &x| x != ' ').is_none() {
			match self.tree.follow(game.x, game.y) {
				None => fail!("Illegal move!"),
				Some(tree) => self.tree = tree
			}
		}
		let (x, y) = self.tree.get_move(self.player);
		match self.tree.follow(x, y) {
			None => fail!("Illegal move!"),
			Some(tree) => self.tree = tree
		}
		game.moveto(x, y);
		game.play();
	}
}
