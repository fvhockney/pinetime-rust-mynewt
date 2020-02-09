use mynewt::{
    self,
    result::*,
    sys::console,
    hw::hal,
    kernel::os::{
        self,
        os_event,
    },
    fill_zero,
};
use core::ffi::c_void;

use crate::backlight::Backlight;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::blocking::delay::DelayMs;
// pull high to enable button
const BUTTON_ENABLE_PIN: i32 = 15; // P0.15

// When high the button is pressed
// When low the button is not pressed
const BUTTON_INDICATOR_PIN: i32 = 13; // P.13

static mut BUTTON_PRESS: os_event = fill_zero!(os_event);

pub fn initialize_button(mut backlight: Backlight) -> MynewtResult<()> {
    let mut button_controll = mynewt::GPIO::new();
    button_controll.init(BUTTON_ENABLE_PIN).expect("Initializing button pin failed");
    button_controll.set_high().unwrap();
    assert_eq!(button_controll.read_state().unwrap(), 1);
    //let button_indicator = mynewt::GPIO::new().init_in(BUTTON_INDICATOR_PIN, mynewt::GpioPullType::DOWN);
    unsafe { BUTTON_PRESS.ev_cb = Some(button_press_callback) };
    let rc = unsafe {
        hal::hal_gpio_irq_init(
            BUTTON_INDICATOR_PIN,
            Some(button_press_handler),
            &mut backlight as *mut _ as *mut c_void,
            // core::ptr::null_mut(),
            hal::hal_gpio_irq_trigger_HAL_GPIO_TRIG_RISING,
            hal::hal_gpio_pull_HAL_GPIO_PULL_DOWN,
        )
    };
    assert_eq!(rc, 0, "IRQ init fail");
    unsafe { hal::hal_gpio_irq_enable(BUTTON_INDICATOR_PIN)};
    // poll(&button_indicator, backlight);
    Ok(())
}

extern "C" fn button_press_handler(arg: *mut core::ffi::c_void) {
    console::print("in handler\n");
    unsafe { BUTTON_PRESS.ev_arg = arg };
    let queue = os::eventq_dflt_get().expect("GET fail");
    unsafe { os::os_eventq_put(queue, &mut BUTTON_PRESS) };
}

extern "C" fn button_press_callback(_event: *mut os_event) {
    let ev = _event;
    let backlight: &mut Backlight = unsafe { &mut *((*ev).ev_arg as *mut Backlight) };
    backlight.increase();
    console::print("callback reacked\n")
}

fn poll (pin: &mynewt::GPIO, backlight: Backlight) -> (){
   loop {
       console::print("polling\n");
       is_button_pressed(pin, backlight);
       mynewt::Delay::new().delay_ms(200);
   } 
}

fn is_button_pressed(pin: &mynewt::GPIO, backlight: Backlight) -> bool {
    let state = pin.read_state().expect("pin state out of range");
    if state == 0 {
        console::print("button not pressed");
        return true
    } else {
        console::print("button pressed");
        backlight.increase();
        return false
    }
}
