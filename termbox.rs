use std::libc::*;
use std::vec::raw::to_ptr;
use std::char::from_u32;
#[deriving(Default)]
pub struct cell { ch: char, fg: u16, bg: u16 }
pub struct tb_cell { ch: uint32_t, fg: uint16_t, bg: uint16_t }
pub struct event { _type: u8, _mod: u8, key: u16, ch: u32, w: i32, h: i32 }
#[deriving(Default)]
pub struct tb_event { _type: uint8_t, _mod: uint8_t, key: uint16_t, ch: uint32_t, w: int32_t, h: int32_t }
pub fn to_cell(c: &tb_cell) -> cell { cell { ch: c.getch(), fg: c.fg as u16, bg: c.bg as u16 } }
impl tb_cell {
	pub fn getch(&self) -> char {
		match from_u32(self.ch as u32) { None => Default::default(), Some(x) => x }
	}
}
impl tb_event {
	pub fn getch(&self) -> char {
		match from_u32(self.ch as u32) { None => Default::default(), Some(x) => x }
	}
}
pub fn to_tb_cell(c: &cell) -> tb_cell { tb_cell { ch: c.ch as uint32_t, fg: c.fg as uint16_t, bg: c.bg as uint16_t } }
pub fn to_event(e: &tb_event) -> event { event { _type: e._type as u8, _mod: e._mod as u8, key: e.key as u16, ch: e.ch as u32, w: e.w as i32, h: e.h as i32 } }
pub fn to_tb_event(e: &event) -> tb_event { tb_event { _type: e._type as uint8_t, _mod: e._mod as uint8_t, key: e.key as uint16_t, ch: e.ch as uint32_t, w: e.w as int32_t, h: e.h as int32_t } }
#[link_args = "-ltermbox"]
extern "C" {
    pub fn tb_init() -> c_int;
    pub fn tb_shutdown();
    pub fn tb_width() -> c_int;
    pub fn tb_height() -> c_int;
    pub fn tb_clear();
    pub fn tb_set_clear_attributes(fg: uint16_t, bg: uint16_t);
    pub fn tb_present();
    pub fn tb_set_cursor(cx: c_int, cy: c_int);
    pub fn tb_put_cell(x: c_int, y: c_int, cell: *tb_cell);
    pub fn tb_change_cell(x: c_int, y: c_int, ch: uint32_t, fg: uint16_t,
                          bg: uint16_t);
    pub fn tb_blit(x: c_int, y: c_int, w: c_int, h: c_int,
                   cells: *tb_cell);
    pub fn tb_select_input_mode(mode: c_int) -> c_int;
    pub fn tb_peek_event(event: *mut tb_event, timeout: c_int) ->
     c_int;
    pub fn tb_poll_event(event: *mut tb_event) -> c_int;
    pub fn tb_utf8_char_length(c: c_schar) -> c_int;
    pub fn tb_utf8_char_to_unicode(out: *mut uint32_t, c: *c_schar) -> c_int;
    pub fn tb_utf8_unicode_to_char(out: *mut c_schar, c: uint32_t) -> c_int;
}

// Key constants. See also struct tb_event's key field.

// These are a safe subset of terminfo keys, which exist on all popular
// terminals. Termbox uses only them to stay truly portable.
pub static KEY_F1: u16 = (0xFFFF-0);
pub static KEY_F2: u16 = (0xFFFF-1);
pub static KEY_F3: u16 = (0xFFFF-2);
pub static KEY_F4: u16 = (0xFFFF-3);
pub static KEY_F5: u16 = (0xFFFF-4);
pub static KEY_F6: u16 = (0xFFFF-5);
pub static KEY_F7: u16 = (0xFFFF-6);
pub static KEY_F8: u16 = (0xFFFF-7);
pub static KEY_F9: u16 = (0xFFFF-8);
pub static KEY_F10: u16 = (0xFFFF-9);
pub static KEY_F11: u16 = (0xFFFF-10);
pub static KEY_F12: u16 = (0xFFFF-11);
pub static KEY_INSERT: u16 = (0xFFFF-12);
pub static KEY_DELETE: u16 = (0xFFFF-13);
pub static KEY_HOME: u16 = (0xFFFF-14);
pub static KEY_END: u16 = (0xFFFF-15);
pub static KEY_PGUP: u16 = (0xFFFF-16);
pub static KEY_PGDN: u16 = (0xFFFF-17);
pub static KEY_ARROW_UP: u16 = (0xFFFF-18);
pub static KEY_ARROW_DOWN: u16 = (0xFFFF-19);
pub static KEY_ARROW_LEFT: u16 = (0xFFFF-20);
pub static KEY_ARROW_RIGHT: u16 = (0xFFFF-21);

// These are all ASCII code points below SPACE character and a BACKSPACE key.
pub static KEY_CTRL_TILDE: u16 = 0x00;
pub static KEY_CTRL_2: u16 = 0x00; // clash with 'CTRL_TILDE'
pub static KEY_CTRL_A: u16 = 0x01;
pub static KEY_CTRL_B: u16 = 0x02;
pub static KEY_CTRL_C: u16 = 0x03;
pub static KEY_CTRL_D: u16 = 0x04;
pub static KEY_CTRL_E: u16 = 0x05;
pub static KEY_CTRL_F: u16 = 0x06;
pub static KEY_CTRL_G: u16 = 0x07;
pub static KEY_BACKSPACE: u16 = 0x08;
pub static KEY_CTRL_H: u16 = 0x08; // clash with 'CTRL_BACKSPACE'
pub static KEY_TAB: u16 = 0x09;
pub static KEY_CTRL_I: u16 = 0x09; // clash with 'TAB'
pub static KEY_CTRL_J: u16 = 0x0A;
pub static KEY_CTRL_K: u16 = 0x0B;
pub static KEY_CTRL_L: u16 = 0x0C;
pub static KEY_ENTER: u16 = 0x0D;
pub static KEY_CTRL_M: u16 = 0x0D; // clash with 'ENTER'
pub static KEY_CTRL_N: u16 = 0x0E;
pub static KEY_CTRL_O: u16 = 0x0F;
pub static KEY_CTRL_P: u16 = 0x10;
pub static KEY_CTRL_Q: u16 = 0x11;
pub static KEY_CTRL_R: u16 = 0x12;
pub static KEY_CTRL_S: u16 = 0x13;
pub static KEY_CTRL_T: u16 = 0x14;
pub static KEY_CTRL_U: u16 = 0x15;
pub static KEY_CTRL_V: u16 = 0x16;
pub static KEY_CTRL_W: u16 = 0x17;
pub static KEY_CTRL_X: u16 = 0x18;
pub static KEY_CTRL_Y: u16 = 0x19;
pub static KEY_CTRL_Z: u16 = 0x1A;
pub static KEY_ESC: u16 = 0x1B;
pub static KEY_CTRL_LSQ_BRACKET: u16 = 0x1B; // clash with 'ESC'
pub static KEY_CTRL_3: u16 = 0x1B; // clash with 'ESC'
pub static KEY_CTRL_4: u16 = 0x1C;
pub static KEY_CTRL_BACKSLASH: u16 = 0x1C; // clash with 'CTRL_4'
pub static KEY_CTRL_5: u16 = 0x1D;
pub static KEY_CTRL_RSQ_BRACKET: u16 = 0x1D; // clash with 'CTRL_5'
pub static KEY_CTRL_6: u16 = 0x1E;
pub static KEY_CTRL_7: u16 = 0x1F;
pub static KEY_CTRL_SLASH: u16 = 0x1F; // clash with 'CTRL_7'
pub static KEY_CTRL_UNDERSCORE: u16 = 0x1F; // clash with 'CTRL_7'
pub static KEY_SPACE: u16 = 0x20;
pub static KEY_BACKSPACE2: u16 = 0x7F;
pub static KEY_CTRL_8: u16 = 0x7F; // clash with 'DELETE'

// Currently there is only one modificator. See also struct tb_event's mod
// field.
pub static MOD_ALT: u8 = 0x01;

// Colors (see struct tb_cell's fg and bg fields).
pub static DEFAULT: u16 = 0x00;
pub static BLACK: u16 = 0x01;
pub static RED: u16 = 0x02;
pub static GREEN: u16 = 0x03;
pub static YELLOW: u16 = 0x04;
pub static BLUE: u16 = 0x05;
pub static MAGENTA: u16 = 0x06;
pub static CYAN: u16 = 0x07;
pub static WHITE: u16 = 0x08;

// Attributes, it is possible to use multiple attributes by combining them
// using bitwise OR ('|'). Although, colors cannot be combined. But you can
// combine attributes and a single color. See also struct tb_cell's fg and bg
// fields.
pub static BOLD: u16 = 0x10;
pub static UNDERLINE: u16 = 0x20;
pub static REVERSE: u16 = 0x40;

pub static EVENT_KEY: u8 = 1;
pub static EVENT_RESIZE: u8 = 2;

pub static EUNSUPPORTED_TERMINAL: int = -1;
pub static EFAILED_TO_OPEN_TTY: int = -2;
pub static EPIPE_TRAP_ERROR: int = -3;

pub static INPUT_CURRENT: int = 0;
pub static INPUT_ESC: int = 1;
pub static INPUT_ALT: int = 2;

pub fn init() -> int {
	#[fixed_stack_segment]; #[inline(never)];
	unsafe { tb_init() as int }
}
pub fn shutdown() {
	#[fixed_stack_segment]; #[inline(never)];
	unsafe { tb_shutdown() }
}
pub fn width() -> int {
	#[fixed_stack_segment]; #[inline(never)];
	unsafe { tb_width() as int }
}
pub fn height() -> int {
	#[fixed_stack_segment]; #[inline(never)];
	unsafe { tb_height() as int }
}
pub fn clear() {
	#[fixed_stack_segment]; #[inline(never)];
	unsafe { tb_clear() }
}
pub fn set_clear_attributes(fg: u16, bg: u16) {
	#[fixed_stack_segment]; #[inline(never)];
	unsafe { tb_set_clear_attributes(fg as uint16_t, bg as uint16_t) }
}
pub fn present() {
	#[fixed_stack_segment]; #[inline(never)];
	unsafe { tb_present() }
}
pub fn set_cursor(cx: int, cy: int) {
	#[fixed_stack_segment]; #[inline(never)];
	unsafe { tb_set_cursor(cx as c_int, cy as c_int ) }
}
pub fn _put_cell(x: int, y: int, cell: *tb_cell) {
	#[fixed_stack_segment]; #[inline(never)];
	unsafe { tb_put_cell(x as c_int, y as c_int, cell) }
}
pub fn put_cell(x: int, y: int, cell: &cell) {
	#[fixed_stack_segment]; #[inline(never)];
	unsafe { tb_put_cell(x as c_int, y as c_int, &to_tb_cell(cell)) }
}
pub fn change_cell(x: int, y: int, ch: char, fg: u16, bg: u16) {
	#[fixed_stack_segment]; #[inline(never)];
	unsafe { tb_change_cell(x as c_int, y as c_int, ch as uint32_t, fg as uint16_t, bg as uint16_t) }
}
pub fn _blit(x: int, y: int, w: int, h: int, cells: *tb_cell) {
	#[fixed_stack_segment]; #[inline(never)];
	unsafe { tb_blit(x as c_int, y as c_int, w as c_int, h as c_int, cells) }
}
pub fn blit(x: int, y: int, cells: ~[~[cell]]) {
	#[fixed_stack_segment]; #[inline(never)];
	let w = cells[0].len() as c_int;
	let h = cells.len() as c_int;
	let ccells = cells.iter().flat_map(|v| v.iter()).map(|e| to_tb_cell(e)).to_owned_vec();
	unsafe { tb_blit(x as c_int, y as c_int, w, h, to_ptr(ccells)) }
}
pub fn select_input_mode(mode: int) -> int {
	#[fixed_stack_segment]; #[inline(never)];
	unsafe { tb_select_input_mode(mode as c_int) as int }
}
pub fn peek_event(event: &mut tb_event, timeout: int) -> int {
	#[fixed_stack_segment]; #[inline(never)];
	unsafe { tb_peek_event(event, timeout as c_int) as int }
}
pub fn poll_event(event: &mut tb_event) -> int {
	#[fixed_stack_segment]; #[inline(never)];
	unsafe { tb_poll_event(event) as int }
}
