pub mod termbox;
pub mod board;
pub mod game;
pub mod ai;

fn main() {
	if termbox::init() < 0 { return; }
	let mut game = game::new(); let mut ai = true;
	let mut ai1 = ai::new('O', 'X'); let mut ai2 = ai::new('X', 'O');
	while game.finished().is_none() { ai = !ai;
		if ai { ai1.move(&mut game); } else { ai2.move(&mut game); }
		std::rt::io::timer::sleep(1000);
	}
	termbox::shutdown();
	game.final();
}
