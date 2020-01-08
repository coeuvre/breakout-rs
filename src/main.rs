mod game;
mod math;
mod software_rendering;

#[cfg(windows)]
mod win32;

fn main() {
    win32::run();
}
