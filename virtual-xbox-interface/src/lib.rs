use winapi::{shared::{minwindef::{BOOL,UCHAR,UINT,INT,BYTE},ntdef::{SHORT}},um::xinput::PXINPUT_VIBRATION};

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
	fn GetLedNumber(UserIndex: UINT, PpLed: BYTE)->BOOL;
	fn GetVibration(UserIndex: UINT, pVib: PXINPUT_VIBRATION)->BOOL;
}

#[derive(Debug)]
pub enum Error{
	UnknownError
}

type Result<T>=std::result::Result<T,Error>;

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
