mod config;

use crate::config::{
	Color, Button
};
use midir;
use std::io::{stdout,Write};
use std::collections::HashMap;
use text_io::*;


fn main() {
	let joystick_port_count=virtual_xbox_interface::get_num_empty_bus_slots().unwrap();
	let joystick_port:usize=(4-joystick_port_count+1).into();
	println!("We have {} virtual joystick ports available, using {}", joystick_port_count, joystick_port);
	virtual_xbox_interface::plug(joystick_port).unwrap();

	let mut midi_in=midir::MidiInput::new("midijoy_i").unwrap();
	println!("We have these MIDI devices now, choose one as input.");
	for idx in 0..midi_in.port_count(){
		println!("\t{}: {}", idx, midi_in.port_name(idx).unwrap());
	}
	print!("Port# ");
	stdout().flush().unwrap();
	let midi_port_in:usize=read!();

	let midi_out=midir::MidiOutput::new("midijoy_o").unwrap();
	println!("We have these MIDI devices now, choose one as output.");
	for idx in 0..midi_out.port_count(){
		println!("\t{}: {}", idx, midi_out.port_name(idx).unwrap());
	}
	print!("Port# ");
	stdout().flush().unwrap();
	let midi_port_out:usize=read!();

	let mut btns=HashMap::new();
	for b in get_buttons(){
		btns.insert(b.id,b.clone());
	}
	dbg!(&btns);

	midi_in.ignore(midir::Ignore::None);
	let mut _conn_in=midi_in.connect(midi_port_in, "m2j_in", move |stamp, message, btns| {
		println!("dbg: {}: {:?} (len = {})", stamp, message, message.len());
		if message[0] == 144{ // Launchpad Mini keys
			let event_key:usize=message[1] as usize;
			// let 
			if !btns.contains_key(&event_key){
				return;
			}
			let btn=btns[&event_key].clone();
			if !btn.persist{
				btn.joystick_button.press(btn.joystick_id,message[2]!=0);
			}
		}
	}, btns).unwrap();
	let mut midi_out=midi_out.connect(midi_port_out, "m2j_out").unwrap();
	midi_out.send(&vec![0x90, 0, 16*3+0+12]).unwrap();
	println!("Connected!");

	let _:String=read!();
    println!("Closing connections.");
	virtual_xbox_interface::unplug_force(joystick_port).unwrap();
}

fn get_buttons()->Vec<config::ButtonConfig>{
	let mut btns:Vec<config::ButtonConfig>=Vec::new();
	btns.push(config::ButtonConfig::new(0,true,1,Button::A,Color::Green));
	btns.push(config::ButtonConfig::new(16,false,1,Button::X,Color::Red));
	btns
}

fn get_led_value(c:Color)->usize{
	1
}
