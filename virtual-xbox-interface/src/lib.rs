use winapi::{shared::{minwindef::{BOOL,UCHAR,UINT,INT,BYTE},ntdef::{SHORT}},um::xinput::XINPUT_VIBRATION};

#[allow(non_camel_case_types)]
#[link(name="vXboxInterface")]
extern {
	// Status
	fn isVBusExists()->BOOL;
	fn GetNumEmptyBusSlots(nSlots: *mut UCHAR)->BOOL;
	fn isControllerExists(UserIndex: UINT)->BOOL;
	fn isControllerOwned(UserIndex: UINT)->BOOL;

	// Virtual device Plug-In/Unplug
	fn PlugIn(UserIndex: UINT)->BOOL;
	fn UnPlug(UserIndex: UINT)->BOOL;
	fn UnPlugForce(UserIndex: UINT)->BOOL;

	// Data Transfer (Data to the device)
	fn SetBtnA(UserIndex: UINT, Press: BOOL)->BOOL;
	fn SetBtnB(UserIndex: UINT, Press: BOOL)->BOOL;
	fn SetBtnX(UserIndex: UINT, Press: BOOL)->BOOL;
	fn SetBtnY(UserIndex: UINT, Press: BOOL)->BOOL;
	fn SetBtnStart(UserIndex: UINT, Press: BOOL)->BOOL;
	fn SetBtnBack(UserIndex: UINT, Press: BOOL)->BOOL;
	fn SetBtnLT(UserIndex: UINT, Press: BOOL)->BOOL; // Left Thumb/Stick
	fn SetBtnRT(UserIndex: UINT, Press: BOOL)->BOOL; // Right Thumb/Stick
	fn SetBtnLB(UserIndex: UINT, Press: BOOL)->BOOL; // Left Bumper
	fn SetBtnRB(UserIndex: UINT, Press: BOOL)->BOOL; // Right Bumper
	fn SetBtnGD(UserIndex: UINT, Press: BOOL)->BOOL; // Guide Button (Undocumanted)
	fn SetTriggerL(UserIndex: UINT, Value: BYTE)->BOOL; // Left Trigger
 	fn SetTriggerR(UserIndex: UINT, Value: BYTE)->BOOL; // Right Trigger
	fn SetAxisX(UserIndex: UINT, Value: SHORT)->BOOL; // Left Stick X
	fn SetAxisY(UserIndex: UINT, Value: SHORT)->BOOL; // Left Stick Y
	fn SetAxisRx(UserIndex: UINT, Value: SHORT)->BOOL; // Right Stick X
	fn SetAxisRy(UserIndex: UINT, Value: SHORT)->BOOL; // Right Stick Y
	fn SetDpadUp(UserIndex: UINT)->BOOL;
	fn SetDpadRight(UserIndex: UINT)->BOOL;
	fn SetDpadDown(UserIndex: UINT)->BOOL;
	fn SetDpadLeft(UserIndex: UINT)->BOOL;
	fn SetDpadOff(UserIndex: UINT)->BOOL;
	fn SetDpad(UserIndex: UINT, Value: INT)->BOOL;

	// Data Transfer (Feedback from the device)
	fn GetLedNumber(UserIndex: UINT, PpLed: *mut BYTE)->BOOL;
	fn GetVibration(UserIndex: UINT, pVib: *mut XINPUT_VIBRATION)->BOOL;
}

#[derive(Debug)]
pub enum Error{
	UnknownError
}
type Result<T>=std::result::Result<T,Error>;

macro_rules! generate_result_fn {
	($fun:ident, $ext:ident, $($var:ident: $rust_type:ty => $c_type:ty), *) => {
		pub fn $fun(user_index: u32, $($var: $rust_type), *)->Result<()>{
			if let 0=unsafe{
				$ext(user_index as UINT, $($var as $c_type), *)
			}{
				Err(Error::UnknownError)
			}else{
				Ok(())
			}
		}
	};
}
macro_rules! generate_ptr_result_fn {
	($fun:ident, $ext:ident, $return_type:ty, $initialize_data:expr) => {
		pub fn $fun(user_index: u32)->Result<$return_type>{
			let mut data: $return_type=$initialize_data;
			if let 0=unsafe{
				$ext(user_index as UINT, &mut data)
			}{
				Err(Error::UnknownError)
			}else{
				Ok(data)
			}
		}
	};
}

// Status
pub fn is_vbus_exists()->bool{unsafe{isVBusExists()!=0}}
pub fn get_num_empty_bus_slots()->Result<u8>{
	let mut slots: u8=0;
	let result=unsafe{GetNumEmptyBusSlots(&mut slots)!=0} as bool;
	if let false=result{
		Err(Error::UnknownError)
	}else{
		Ok(slots)
	}
}
pub fn is_controller_exists(user_index: u32)->bool{unsafe{isControllerExists(user_index as UINT)!=0}}
pub fn is_controller_owned(user_index: u32)->bool{unsafe{isControllerOwned(user_index as UINT)!=0}}

// Virtual device Plug-In/Unplug
generate_result_fn!(plug, PlugIn,);
generate_result_fn!(unplug, UnPlug,);
generate_result_fn!(unplug_force, UnPlugForce,);

// Data Transfer (Data to the device)
generate_result_fn!(set_button_a, SetBtnA, press: bool=>BOOL);
generate_result_fn!(set_button_b, SetBtnB, press: bool=>BOOL);
generate_result_fn!(set_button_x, SetBtnX, press: bool=>BOOL);
generate_result_fn!(set_button_y, SetBtnY, press: bool=>BOOL);
generate_result_fn!(set_button_lt, SetBtnLT, press: bool=>BOOL); // Left Thumb/Stick
generate_result_fn!(set_button_rt, SetBtnRT, press: bool=>BOOL); // Right Thumb/Stick
generate_result_fn!(set_button_lb, SetBtnLB, press: bool=>BOOL); // Left Bumper
generate_result_fn!(set_button_rb, SetBtnRB, press: bool=>BOOL); // Right Bumper
generate_result_fn!(set_button_gd, SetBtnGD, press: bool=>BOOL); // Guide Button (Undocumanted)
generate_result_fn!(set_button_start, SetBtnStart, press: bool=>BOOL);
generate_result_fn!(set_button_back, SetBtnBack, press: bool=>BOOL);
generate_result_fn!(set_trigger_l, SetTriggerL, value: u8=>BYTE); // Left Trigger
generate_result_fn!(set_trigger_r, SetTriggerR, value: u8=>BYTE); // Right Trigger
generate_result_fn!(set_axis_lx, SetAxisX, value: i16=>SHORT); // Left Stick X
generate_result_fn!(set_axis_ly, SetAxisY, value: i16=>SHORT); // Left Stick Y
generate_result_fn!(set_axis_rx, SetAxisRx, value: i16=>SHORT); // Right Stick X
generate_result_fn!(set_axis_ry, SetAxisRy, value: i16=>SHORT); // Right Stick Y
generate_result_fn!(set_dpad, SetDpad, value: i32=>INT);
generate_result_fn!(set_dpad_off, SetDpadOff,);
generate_result_fn!(set_dpad_up, SetDpadUp,);
generate_result_fn!(set_dpad_down, SetDpadDown,);
generate_result_fn!(set_dpad_left, SetDpadLeft,);
generate_result_fn!(set_dpad_right, SetDpadRight,);

// Data Transfer (Feedback from the device)
generate_ptr_result_fn!(get_led_number, GetLedNumber, u8, 0);
generate_ptr_result_fn!(get_vibration, GetVibration, XINPUT_VIBRATION, XINPUT_VIBRATION{
	wLeftMotorSpeed: 0,
	wRightMotorSpeed: 0
});
