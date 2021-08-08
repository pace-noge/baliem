use super::*;

pub(super) struct off;

impl Off {
    pub fn new() -> self {
        Off
    }
}

imple LEDMode for Off {
    fn next_matrix(&mut self, _last: LEDMatrix) -> Option<LEDMatrix> {
        Some(LEDMatrix {
            // keys: [[(0, 0, 0).into(); 7]; 3],
            // thumb: [(0, 0, 0).into(); 5],
            underglow: [(0, 0, 0).into(); 6],
        })
    }
}