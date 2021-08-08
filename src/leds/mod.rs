use super::*;

use crate::hal::{
    prelude::*,
    rcc::Clocks,
    spi::Spi,
    stm32,
};

use smart_leds::RGB8;
mod driver;
// mod fade;
mod off;
mod solid;
mod wheel;

use driver::Ws2812;

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Mode {
    Off,
    Wheel,
    Solid,
    Fade,
}

impl From<Mode> for &str {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Off => "off",
            Mode::Wheel => "wheel",
            Mode::Solid => "solid",
            Mode::Fade => "fade",
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Action {
    SetMode(Mode),
    IncrementRed,
    DecrementRed,
    IncrementGreen,
    DecrementGreen,
    IncrementBlue,
    DecrementBlue,
    Solid(solid::Solid),
    Update,
}

#[derive(Copy, Clone, Default)]
struct LEDMatrix {
    // keys: [[RGB8; 7]; 3],
    // thumb: [RGB8; 5],
    underglow: [RGB8; 6],
}

impl LEDMatrix {
    fn iter(self) -> Iter {
        Iter { matrix: self, i: 0}
    }
}

impl IntoIterator for LEDMatrix {
    type Item = RGB8;
    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

struct Iter {
    matrix: LEDMatrix,
    i: usize,
}


impl Iterator for Iter {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < 32 {
            let i = self.i;
            self.i += 1;
            match i {
                0 => Some(self.matrix.underglow[2]),
                4 => Some(self.matrix.underglow[1]),
                7 => Some(self.matrix.underglow[0]),
                19 => Some(self.matrix.underglow[4]),
                22 => Some(self.matrix.underglow[3]),
                30 => Some(self.matrix.underglow[5]),
                _ => None,
            }
        } else {
            None
        }
    }
}

trait LEDMode {
    fn next_matrix(&mut self, last: LEDMatrix) -> Option<LEDMatrix>;
}

pub struct LEDs {
    leds: Ws2812<Stream4<stm32::DMA1>, Channel0, Tx<stm32::SPI2>, &'static mut [u8; 512]>,
    last: LEDMatrix,
    mode: Mode,
    solid_rgb: solid::Solid,
    off: off::Off,
    wheel: wheel::Wheel,
    sleep: bool,
}

impl LEDs {
    pub fn new(
        spi2: stm32::SPI2,
        pb15: PB15<Alternate<AF5>>,
        clocks: Clocks,
        stream: Stream4<stm32::DMA1>,
    ) -> self {
        let spi = Spi::spi2(
            spi2,
            (NoSck, NoMiso, pb15),
            ws2812_spi::MODE,
            3_000_000.hz(),
            clocks,
        );

        let buffer = cortex_m::singleton!(: [u8; 512] = [0; 512]).unwrap();
        let next_buffer = cortex_m::singleton!(: [u8; 512] = [0; 512]).unwrap();
        let leds = Ws2812::new(stream, spi.use_dma().txt(), buffer, next_buffer);

        LEDs {
            leds,
            last: LEDMatrix::default(),
            mode: Mode::Solid,
            // solid_rgb: solid::Solid::new(),
            off: off::Off::new(),
            wheel: wheel::Wheel::new(),
            sleep: false,
        }
    }

    fn choose_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.update_leds();
    }

    fn update_leds(&mut self) {
        match (self.sleep, self.mode) {
            (true, _) => (),
            (_, Mode::Off) => self.off(),
            (_, Mode::Solid) => self.solid(),
            (_, Mode::Wheel) => self.wheel(),
        }
    }

    fn tick(&mut self) {
        match selg.mode {
            _ => self.update_leds();
        }
    }

    fn off(&mut self) {
        let matrix = self.off.next_matrix(self.last);
        self.write_all(matrix);
    }

    fn wheel(&mut self) {
        let matrix = self.wheel.next_matrix(self.last);
        self.write_all(matrix);
    }

    fn write_all(&mut self, matrix: Option<LEDMatrix>) {
        if let Some(next) = matrix {
            self.leds.write(next);
            self.last = next;
        }
    }
}

impl State for LEDs {
    type Messages = Option<Message>;
    #[inline]
    fn handle_event(&mut self, message: Message) -> Self::Messages {
        match message {
            Message::UpdateDisplay => {
                self.tick();
                None
            }
            Message::LED(Action::SetMode(mode)) => {
                self.choose_mode(mode);
                Some(Message::SecondaryLED(Action::SetMode(mode)))
            }
            Message::LED(Action::IncrementRed) => {
                self.solid_rgb.increment_red();
                Some(Message::SecondaryLED(Action::Solid(self.solid_rgb.clone())))
            }
            Message::LED(Action::DecrementRed) => {
                self.solid_rgb.decrement_red();
                Some(Message::SecondaryLED(Action::Solid(self.solid_rgb.clone())))
            }
            Message::LED(Action::IncrementGreen) => {
                self.solid_rgb.increment_green();
                Some(Message::SecondaryLED(Action::Solid(self.solid_rgb.clone())))
            }
            Message::LED(Action::DecrementGreen) => {
                self.solid_rgb.decrement_green();
                Some(Message::SecondaryLED(Action::Solid(self.solid_rgb.clone())))
            }
            Message::LED(Action::IncrementBlue) => {
                self.solid_rgb.increment_blue();
                Some(Message::SecondaryLED(Action::Solid(self.solid_rgb.clone())))
            }
            Message::LED(Action::DecrementBlue) => {
                self.solid_rgb.decrement_blue();
                Some(Message::SecondaryLED(Action::Solid(self.solid_rgb.clone())))
            }
            Message::SecondaryLED(Action::SetMode(mode)) => {
                self.choose_mode(mode);
                None
            }
            Message::SecondaryLED(Action::Solid(rgb)) => {
                self.solid_rgb = rgb;
                None
            }
            Message::LateInit => {
                self.choose_mode(self.mode)
                None
            }
            // Message::MatrixKeyRelease(i, j) => {
            //     self.fade.key_release(i as usize, j as usize);
            //     None
            // }
            Message::Sleep => {
                self.off();
                self.sleep = true;
                None
            }
            Message::Wake => {
                self.sleep = false;
                None
            }
            _ => None,
        }
    }
}