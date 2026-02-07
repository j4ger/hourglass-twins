#![no_main]
#![no_std]

use rmk::macros::rmk_central;

// Create and run your keyboard with a single macro: `rmk_keyboard`, that's it!
#[rmk_central]
mod keyboard_central {}
