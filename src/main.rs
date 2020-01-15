pub mod game;
pub mod index_vec;
pub mod input;
pub mod line;
pub mod math;
pub mod software_rendering;

#[cfg(windows)]
mod win32;

fn main() {
    win32::run();
}
