#[derive(Debug)]
pub enum Color{
	Red=1,
	Green=2,
	Yellow=3
}

#[derive(Debug)]
pub struct ButtonConfig{
	pub id: usize,
	pub persist: bool,
	pub joystick_id: usize,
	pub joystick_button: usize,
	pub color: Color
}
impl ButtonConfig{
	pub fn new(id: usize,persist: bool,joy_id: usize,joy_button: usize,color: Color)->Self{
		ButtonConfig{
			id: id,
			persist: persist,
			joystick_id: joy_id,
			joystick_button: joy_button,
			color: color
		}
	}
}
