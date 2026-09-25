// use serde::{Deserialize, Serialize};
//
// use super::EffectState;
// use crate::Colour;
// use crate::keyboard::{KeyLayout, LedCode};
//
// // fn get_colour_num(colour: &mut Colour, colour_val: usize) -> u8 {
//     if colour_val == 0 {
//         colour.r
//     } else if colour_val == 1 {
//         colour.g
//     } else {
//         colour.b
//     }
// }
//
// fn set_colour_num(colour: &mut Colour, colour_val: usize, val: u8) -> &mut Colour {
//     if colour_val == 0 {
//         set_red(colour, val)
//     } else if colour_val == 1 {
//         set_green(colour, val)
//     } else {
//         set_blue(colour, val)
//     }
// }
//
// fn set_red(colour: &mut Colour, num: u8) -> &mut Colour {
//     colour.r = num;
//     colour
// }
//
// fn set_green(colour: &mut Colour, num: u8) -> &mut Colour {
//     colour.g = num;
//     colour
// }
//
// fn set_blue(colour: &mut Colour, num: u8) -> &mut Colour {
//     colour.b = num;
//     colour
// }

// #[derive(Debug, Clone, Deserialize, Serialize)]
//
// pub struct Rainbow {
//     led: LedCode,
//     /// Temporary data to help keep state
//     #[serde(skip)]
//     colour: Colour,
//     #[serde(skip)]
//     cur_change: usize,
//     //#[serde(skip)]
//     //curr_chang_prev: usize,
//     //#[serde(skip)]
//     //skip_first: bool,
// }
// impl Rainbow {
//     pub fn new(address: LedCode) -> Self {
//         Self {
//             led: address,
//             colour: Colour {
//                 r: (255),
//                 g: (0),
//                 b: (0),
//             },
//             cur_change: 0,
//             //curr_chang_prev: 0,
//             //skip_first: true,
//         }
//     }
// }
//
// impl EffectState for Rainbow {
//     fn get_colour(&self) -> Colour {
//         self.colour
//     }
//
//     fn get_led(&self) -> LedCode {
//         self.led
//     }
//
//     fn set_led(&mut self, address: LedCode) {
//         self.led = address;
//     }
//     fn next_colour_state(&mut self, _layout: &KeyLayout) {
//         //let Self { colour, .. } = self;
//         if self.cur_change == 2 {
//             self.cur_change = 0;
//         } else {
//             self.cur_change += 1;
//         }
//     }
// }]

fn main() {
    println!("Hello");
    println!("I jusr want this to compile dammit")
}
