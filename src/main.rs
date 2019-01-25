use midir;
use std::io::{stdout,Write};
use text_io::*;
use virtual_xbox_interface;

fn main() {
	let joystick_port_count=virtual_xbox_interface::get_num_empty_bus_slots().unwrap();
	let joystick_port:u32=(4-joystick_port_count+1).into();
	println!("We have {} virtual joystick ports available, using {}", joystick_port_count, joystick_port);
	virtual_xbox_interface::plug(joystick_port).unwrap();

	let midi_in=midir::MidiInput::new("midijoy").unwrap();
	println!("We have these MIDI devices now, choose one as input.");
	for idx in 0..midi_in.port_count(){
		println!("\t{}: {}", idx, midi_in.port_name(idx).unwrap());
	}
	print!("Port# ");
	stdout().flush().unwrap();
	let midi_port:usize=read!();
	midi_in.connect(midi_port, "__in__", move |stamp, message, _| {
		if message[0]==176{ // LK CONTROLLER
			match message[1]{
				3=>{virtual_xbox_interface::set_button_a(joystick_port, message[2]!=0).unwrap();}
				7=>{virtual_xbox_interface::set_button_b(joystick_port, message[2]!=0).unwrap();}
				11=>{virtual_xbox_interface::set_button_x(joystick_port, message[2]!=0).unwrap();}
				15=>{virtual_xbox_interface::set_button_y(joystick_port, message[2]!=0).unwrap();}
				2=>{virtual_xbox_interface::set_axis_lx(joystick_port, ((message[2] as f32)/128.0*65536.0-32768.0) as i16).unwrap();}
				6=>{virtual_xbox_interface::set_axis_ly(joystick_port, ((message[2] as f32)/128.0*65536.0-32768.0) as i16).unwrap();}
				10=>{virtual_xbox_interface::set_axis_rx(joystick_port, ((message[2] as f32)/128.0*65536.0-32768.0) as i16).unwrap();}
				14=>{virtual_xbox_interface::set_axis_ry(joystick_port, ((message[2] as f32)/128.0*65536.0-32768.0) as i16).unwrap();}
				n=>{println!("Unexpected key: {}.\t{}: {:?} (len = {})", n, stamp, message, message.len());}
			};
		}
	}, ()).unwrap();

	let _:String=read!();
    println!("Closing connections.");
	virtual_xbox_interface::unplug_force(joystick_port).unwrap();
}
