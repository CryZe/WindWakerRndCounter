#![no_std]
use core::{
    fmt::Write,
    sync::atomic::{AtomicUsize, Ordering},
};
use libtww::{game::Console, system::custom_game_loop};

pub mod panic;

static COUNT: AtomicUsize = AtomicUsize::new(0);

#[repr(C)]
struct RngState {
    r0: usize,
    r1: usize,
    r2: usize,
}

#[no_mangle]
pub extern "C" fn count_call(rnd: f64) -> f64 {
    let count = COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    let address: usize = if cfg!(feature = "ntsc_u") {
        0x803f7338
    } else {
        0x803ea7d8
    };
    let state = unsafe { &*(address as *mut RngState) };
    let _ = write!(Console::get().lines[0].begin(), "Count: {}", count);
    let _ = write!(Console::get().lines[2].begin(), "R0: {:>5}", state.r0);
    let _ = write!(Console::get().lines[3].begin(), "R1: {:>5}", state.r1);
    let _ = write!(Console::get().lines[4].begin(), "R2: {:>5}", state.r2);
    rnd
}

#[no_mangle]
pub extern "C" fn init_board() {
    let _ = write!(
        Console::get().lines[6].begin(),
        "Board Init Count: {}",
        COUNT.load(Ordering::Relaxed)
    );
}

#[inline(never)]
fn init() {
    let console = Console::get();
    console.line_count = 32;
    console.x = 0;
    console.y = 16;
    console.font_scale_x *= 1.2;
    console.font_scale_y *= 1.2;
    console.background_color.a = 0;
    console.clear();
    console.visible = true;
}

#[no_mangle]
pub extern "C" fn game_loop() -> ! {
    init();
    custom_game_loop(|| {})
}
