use virtual_xbox_interface;

fn main() {
	dbg!(virtual_xbox_interface::plug(1));
	dbg!(virtual_xbox_interface::is_controller_exists(1));
	dbg!(virtual_xbox_interface::is_controller_owned(1));

	std::thread::sleep_ms(5000);
	dbg!(virtual_xbox_interface::set_button_a(1,true));
	std::thread::sleep_ms(1000);
	dbg!(virtual_xbox_interface::set_button_x(1,true));
	std::thread::sleep_ms(1000);
	dbg!(virtual_xbox_interface::set_button_x(1,false));
	std::thread::sleep_ms(1000);
	dbg!(virtual_xbox_interface::set_button_a(1,false));

	std::thread::sleep_ms(5000);
	dbg!(virtual_xbox_interface::unplug_force(1));
}
