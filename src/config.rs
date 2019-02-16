use virtual_xbox_interface;

#[derive(Debug,Clone)]
pub enum Color{
	Red=1,
	Green=2,
	Yellow=3
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
			A=>virtual_xbox_interface::set_button_a(port, toggle).unwrap(),
			X=>virtual_xbox_interface::set_button_x(port, toggle).unwrap(),
			_n=>()
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
