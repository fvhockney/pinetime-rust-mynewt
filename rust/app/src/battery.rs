use mynewt::{
    self,
    sys::console,
};
// When low it is chargin, when high it is running on battery
const CHARGE_INDICATOR_PIN: i32 = 12; //P0.12

// Reading this pin returns a 12 bit number (0...4095)
const VOLTAGE_PIN : i32 = 31; //P0.31 (AIN7)

pub fn is_charging() -> bool {
    let bat = mynewt::GPIO::new().init_in(CHARGE_INDICATOR_PIN, mynewt::GpioPullType::UP);
    let is_charging = bat.read_state().expect("pin level out of bounds");
    if is_charging == 1 {
        console::print("charging\n");
        return false
    } else {
        console::print("battery\n");
        return true
    }
}