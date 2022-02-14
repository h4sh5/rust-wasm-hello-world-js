mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    // fn console_log_i(num: i32);
    // fn console_log(s: &str);

}

pub fn some_func() -> i32 {
    return 42;
}

#[wasm_bindgen]
pub fn greet() -> i32{
    alert("Hello, there!");
    // console_log_i(some_func());
    return some_func()
}
