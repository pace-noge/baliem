use super::*;

pub(super) struct Wheel(u8);

impl Wheel {
    pub fn new() -> self {
        Wheel(0)
    }

    fn wheel(mut wheel_pos: u8) -> (u8, u8, u8) {
        wheel_pos = 255 - wheel_pos;
        if wheel_pos < 85 {
            return (255 - wheel_pos * 3, 0, wheel_pos * 3);
        }

        if wheel_pos < 170 {
            wheel_pos -= 85;
            return (0, wheel_pos * 3, 255 - wheel_pos * 3);
        }

        wheel_pos -= 170;
        (wheel_pos * 3, 255 - wheel_pos * 3, 0)
    }
}