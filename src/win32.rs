#![windows_subsystem = "windows"]

extern crate winapi;

use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::memoryapi::{VirtualAlloc, VirtualFree};
use winapi::um::profileapi::*;
use winapi::um::wingdi::*;
use winapi::um::winnt::*;
use winapi::um::winuser::*;

use crate::game::*;
use crate::input::keyboard::*;
use crate::input::mouse::Button;
use crate::input::Input;
use crate::software_rendering::*;

struct Win32RenderBuffer {
    width: i32,
    height: i32,
    pixels: *mut u32,
    bitmap_info: BITMAPINFO,
}

static mut RUNNING: bool = true;
static mut RENDER_BUFFER: *mut Win32RenderBuffer = std::ptr::null_mut();

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let render_buffer = &mut *RENDER_BUFFER;

    match msg {
        WM_CLOSE | WM_DESTROY => {
            RUNNING = false;
        }
        WM_SIZE => {
            let mut rect = std::mem::MaybeUninit::uninit();
            GetClientRect(hwnd, rect.as_mut_ptr());
            let rect = rect.assume_init();

            render_buffer.width = rect.right - rect.left;
            render_buffer.height = rect.bottom - rect.top;

            if render_buffer.pixels != std::ptr::null_mut() {
                VirtualFree(render_buffer.pixels as LPVOID, 0, MEM_RELEASE);
            }

            render_buffer.pixels = VirtualAlloc(
                0 as LPVOID,
                std::mem::size_of::<u32>()
                    * render_buffer.width as usize
                    * render_buffer.height as usize,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            ) as *mut u32;

            render_buffer.bitmap_info.bmiHeader.biSize =
                std::mem::size_of_val(&render_buffer.bitmap_info.bmiHeader) as u32;
            render_buffer.bitmap_info.bmiHeader.biWidth = render_buffer.width;
            render_buffer.bitmap_info.bmiHeader.biHeight = render_buffer.height;
            render_buffer.bitmap_info.bmiHeader.biPlanes = 1;
            render_buffer.bitmap_info.bmiHeader.biBitCount = 32;
            render_buffer.bitmap_info.bmiHeader.biCompression = BI_RGB;
        }
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }

    0
}

fn wstr(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

unsafe fn do_run() {
    let render_buffer = &mut *RENDER_BUFFER;

    let mut window_class = std::mem::zeroed::<WNDCLASSW>();
    window_class.style = CS_HREDRAW | CS_VREDRAW;
    window_class.lpfnWndProc = Some(window_proc);
    let class_name = wstr("GAME_WINDOW_CLASS");
    window_class.lpszClassName = class_name.as_ptr();

    RegisterClassW(&mut window_class);

    let window_name = wstr("Breakout");
    let hwnd = CreateWindowExW(
        0,
        window_class.lpszClassName,
        window_name.as_ptr(),
        WS_VISIBLE | WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        1280,
        720,
        0 as HWND,
        0 as HMENU,
        0 as HINSTANCE,
        0 as LPVOID,
    );
    let hdc = GetDC(hwnd);

    let mut game = Game::new();
    let mut input = Input::new();

    let mut last_counter = std::mem::zeroed();
    QueryPerformanceCounter(&mut last_counter);

    let mut frequency_counter = std::mem::zeroed();
    QueryPerformanceFrequency(&mut frequency_counter);

    let mut last_dt = 0.01666;

    while RUNNING {
        input.keyboard.keys.iter_mut().for_each(|key| {
            key.was_down = false;
        });

        input.mouse.buttons.iter_mut().for_each(|button| {
            button.was_down = button.is_down;
        });

        let mut msg = std::mem::MaybeUninit::<MSG>::uninit();
        while PeekMessageW(msg.as_mut_ptr(), hwnd, 0 as UINT, 0 as UINT, PM_REMOVE) != 0 {
            let msg = msg.assume_init();
            match msg.message {
                WM_LBUTTONDOWN => {
                    input.mouse.button_mut(Button::Left).is_down = true;
                }
                WM_LBUTTONUP => {
                    input.mouse.button_mut(Button::Left).is_down = false;
                }
                WM_RBUTTONDOWN => {
                    input.mouse.button_mut(Button::Right).is_down = true;
                }
                WM_RBUTTONUP => {
                    input.mouse.button_mut(Button::Right).is_down = false;
                }

                WM_SYSKEYDOWN | WM_SYSKEYUP | WM_KEYDOWN | WM_KEYUP => {
                    let vk_code = msg.wParam as i32;
                    let was_down = (msg.lParam & (1 << 30)) != 0;
                    let is_down = (msg.lParam & (1 << 31)) == 0;

                    macro_rules! process_key {
                        ($vk:expr, $b:expr) => {
                            if vk_code == $vk {
                                let key = &mut input.keyboard.key_mut($b);
                                key.is_down = is_down;
                                key.was_down = was_down;
                            }
                        };
                    }
                    process_key!(VK_LEFT, Key::Left);
                    process_key!(VK_RIGHT, Key::Right);
                    process_key!(VK_UP, Key::Up);
                    process_key!(VK_DOWN, Key::Down);
                }
                _ => {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }

        let mut mouse_pointer = std::mem::zeroed();
        GetCursorPos(&mut mouse_pointer);
        ScreenToClient(hwnd, &mut mouse_pointer);

        input.mouse.position.x = mouse_pointer.x as f32;
        input.mouse.position.y = (render_buffer.height - mouse_pointer.y) as f32;

        // Simulation
        {
            let mut render_buffer = RenderBuffer {
                pixels: std::slice::from_raw_parts_mut(
                    render_buffer.pixels,
                    (render_buffer.width * render_buffer.height) as usize,
                ),
                width: render_buffer.width,
                height: render_buffer.height,
            };

            game.simulate(&mut render_buffer, &input, last_dt);
        }

        // Render
        StretchDIBits(
            hdc,
            0,
            0,
            render_buffer.width,
            render_buffer.height,
            0,
            0,
            render_buffer.width,
            render_buffer.height,
            render_buffer.pixels as LPVOID,
            &mut render_buffer.bitmap_info,
            DIB_RGB_COLORS,
            SRCCOPY,
        );

        let mut current_counter = std::mem::zeroed();
        QueryPerformanceCounter(&mut current_counter);

        last_dt = (current_counter.QuadPart() - last_counter.QuadPart()) as f32
            / *frequency_counter.QuadPart() as f32;

        last_counter = current_counter;
    }
}

pub fn run() {
    unsafe {
        let mut render_buffer = std::mem::zeroed::<Win32RenderBuffer>();
        RENDER_BUFFER = &mut render_buffer;

        do_run();
    }
}
