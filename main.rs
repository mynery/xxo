pub mod termbox;
pub mod board;
pub mod game;
pub mod ai;

fn main() {
	if termbox::init() < 0 { return; }
	let mut game = game::new();
	let mut event: termbox::tb_event = Default::default();
	let mut ai = ai::new('O', 'X');
	while event.getch() != 'q' && game.finished().is_none() {
		termbox::poll_event(&mut event);
		if event.key == termbox::KEY_ENTER && game.play() {
			ai.move(&mut game);
		}
		game.move(event.key);
	}
	termbox::shutdown();
	game.final();
}
