#![no_std]
use core::{
    fmt::Write,
    sync::atomic::{AtomicUsize, Ordering},
};
use libtww::{game::Console, system::custom_game_loop};

pub mod panic;

const RATE_WINDOW_SIZE: usize = 30; // use framerate to make it 1s avg

static RNG_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

enum ConsoleLines {
    RngCallCount = 0,
    RngState0 = 2,
    RngState1 = 3,
    RngState2 = 4,
    BoardInit = 5,
    RngCallRateFrame = 6,
    RngCallRateAvg = 7,
}

#[repr(C)]
struct RngState {
    r0: usize,
    r1: usize,
    r2: usize,
}

#[no_mangle]
pub extern "C" fn count_call(rnd: f64) -> f64 {
    let count = RNG_CALL_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    let address: usize = if cfg!(feature = "ntsc_u") {
        0x803f7338
    } else {
        0x803ea7d8
    };
    let state = unsafe { &*(address as *mut RngState) };
    let _ = write!(
        Console::get().lines[ConsoleLines::RngCallCount as usize].begin(), 
        "Count: {}", 
        count
    );
    let _ = write!(
        Console::get().lines[ConsoleLines::RngState0 as usize].begin(), 
        "R0: {:>5}", 
        state.r0
    );
    let _ = write!(
        Console::get().lines[ConsoleLines::RngState1 as usize].begin(),
         "R1: {:>5}", 
         state.r1
    );
    let _ = write!(
        Console::get().lines[ConsoleLines::RngState2 as usize].begin(), 
        "R2: {:>5}", 
        state.r2
    );
    rnd
}

#[no_mangle]
pub extern "C" fn init_board() {
    let _ = write!(
        Console::get().lines[ConsoleLines::BoardInit as usize].begin(),
        "Board Init Count: {}",
        RNG_CALL_COUNT.load(Ordering::Relaxed)
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

    let mut frame_count: u32 = 0;
    let mut current_rng_call_count: usize = 0;
    let mut rolling_arr_index = 0;
    let mut rate_avg_window: [usize; RATE_WINDOW_SIZE] = [0; RATE_WINDOW_SIZE];
    custom_game_loop(|| {
        frame_count += 1;
        let prev_rng_call_count = core::mem::replace(
            &mut current_rng_call_count, 
            RNG_CALL_COUNT.load(Ordering::Relaxed)
        );
        let rng_call_diff = current_rng_call_count - prev_rng_call_count;
        rolling_arr_index = (rolling_arr_index + 1) % RATE_WINDOW_SIZE;
        rate_avg_window[rolling_arr_index] = rng_call_diff;
        let calls_mean: f64 = (rate_avg_window.iter().sum::<usize>() as f64) / (RATE_WINDOW_SIZE as f64);
        let _ = write!(
            Console::get().lines[ConsoleLines::RngCallRateFrame as usize].begin(), 
            "RNG Calls: {}",
             rng_call_diff
        );
        let _ = write!(
            Console::get().lines[ConsoleLines::RngCallRateAvg as usize].begin(), 
            "RNG Calls Avg: {:.2}",
            calls_mean
        );
    })
}
