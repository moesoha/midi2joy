use virtual_xbox_interface;

#[derive(Debug,Clone)]
pub enum Color{
	Red=1,
	Green=2,
	Yellow=3
}
impl Color{
	pub fn set_key_brightness(&self, midi_out: &mut midir::MidiOutputConnection, key: u8, brightness: u8)->(){
		match self{
			Color::Red=>midi_out.send(&vec![0x90, key, brightness+12]).unwrap(),
			Color::Green=>midi_out.send(&vec![0x90, key, 16*brightness+0+12]).unwrap(),
			Color::Yellow=>midi_out.send(&vec![0x90, key, 16*brightness+brightness+12]).unwrap(),
		}
	}
}

#[derive(Debug,Clone)]
pub enum Button{
	A=1,
	B=2,
	X=3,
	Y=4,
	Start=5,
	Back=6,
	LT=7,
	RT=8,
	LB=9,
	RB=10,
	GD=11
}
impl Button{
	pub fn press(&self, port: usize, toggle: bool)->(){
		match self{
			Button::A=>virtual_xbox_interface::set_button_a(port, toggle).unwrap(),
			Button::B=>virtual_xbox_interface::set_button_b(port, toggle).unwrap(),
			Button::X=>virtual_xbox_interface::set_button_x(port, toggle).unwrap(),
			Button::Y=>virtual_xbox_interface::set_button_y(port, toggle).unwrap(),
			Button::Start=>virtual_xbox_interface::set_button_start(port, toggle).unwrap(),
			Button::Back=>virtual_xbox_interface::set_button_back(port, toggle).unwrap(),
			Button::LT=>virtual_xbox_interface::set_button_lt(port, toggle).unwrap(),
			Button::RT=>virtual_xbox_interface::set_button_rt(port, toggle).unwrap(),
			Button::LB=>virtual_xbox_interface::set_button_lb(port, toggle).unwrap(),
			Button::RB=>virtual_xbox_interface::set_button_rb(port, toggle).unwrap(),
			Button::GD=>virtual_xbox_interface::set_button_gd(port, toggle).unwrap()
		}
	}
}

#[derive(Debug,Clone)]
pub struct ButtonConfig{
	pub id: usize,
	pub persist: bool,
	pub joystick_id: usize,
	pub joystick_button: Button,
	pub color: Color
}
impl ButtonConfig{
	pub fn new(id: usize,persist: bool,joy_id: usize,joy_button: Button,color: Color)->Self{
		ButtonConfig{
			id: id,
			persist: persist,
			joystick_id: joy_id,
			joystick_button: joy_button,
			color: color
		}
	}
}
