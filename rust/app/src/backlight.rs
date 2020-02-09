#![no_std]
use mynewt::{
    self,
    result::*,
    sys::console,
    hw::hal,
};
use mynewt::kernel::os::os_event;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::blocking::delay::DelayMs;

const BACKLIGHT_PIN_1: i32 = 14;
const BACKLIGHT_PIN_2: i32 = 22;
const BACKLIGHT_PIN_3: i32 = 23;

#[derive(Copy,Clone)]
enum Level {
    LOW,
    MEDIUM,
    HIGH,
}

#[derive(Copy,Clone)]
pub struct Backlight {
    level: Level,
    low_pin: mynewt::GPIO,
    medium_pin: mynewt::GPIO,
    high_pin: mynewt::GPIO,
}

impl Backlight {
    pub fn init() -> Backlight {
        let bl = Backlight {
            low_pin: mynewt::GPIO::new().init_out(BACKLIGHT_PIN_1),
            medium_pin: mynewt::GPIO::new().init_out(BACKLIGHT_PIN_2),
            high_pin: mynewt::GPIO::new().init_out(BACKLIGHT_PIN_3),
            level: Level::LOW
        };
        bl.set(Level::LOW)
    }

    fn set (mut self, new_level: Level) -> Self {
        match new_level {
            Level::LOW => {
                self.level = Level::LOW;
                self.medium_pin.set_high().unwrap();
                self.high_pin.set_high().unwrap();
            },
            Level::MEDIUM => {
                self.level = Level::MEDIUM;
                self.medium_pin.set_low().unwrap();
                self.high_pin.set_high().unwrap();
            },
            Level::HIGH => {
                self.level = Level::HIGH;
                self.high_pin.set_low().unwrap();
            },
        };
        self
    }

    pub fn get_current_level(&self) -> Level {
        let low_state = self.low_pin.read_state().expect("read falied");
        let medium_state = self.medium_pin.read_state().expect("read falied");
        let high_state = self.high_pin.read_state().expect("read falied");
        if ( high_state == 0) {
            Level::HIGH
        } else if ( medium_state == 0 ) {
            Level::MEDIUM
        } else {
            Level::LOW
        }
    }

    pub fn increase(mut self) -> Self {
        let level = self.get_current_level();
        match level {
            Level::LOW => {
                self.level = Level::MEDIUM;
                self.set(Level::MEDIUM)
            },
            Level::MEDIUM => {
                self.level = Level::HIGH;
                self.set(Level::HIGH)
            },
            Level::HIGH => {
                self.level = Level::LOW;
                self.set(Level::LOW)
            },
        };
        self
    }

    //fn decrease(mut self) -> Self {
    //    self.level = match self.level {
    //        Level::LOW => Level::HIGH.set(&mut self),
    //        Level::MEDIUM => Level::LOW.set(&mut self),
    //        Level::HIGH => Level::MEDIUM.set(&mut self),
    //    };
    //    self
    //}
}