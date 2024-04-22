#![no_std]

extern crate wee_alloc;


use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // Use `web_sys` to interact with the DOM.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let title = document.create_element("h1")?;
    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    title.set_inner_html("WEBPAGE FROM WASM:");
    val.set_inner_html("Hello from Rust!");

    body.append_child(&title)?;
    body.append_child(&val)?;

    Ok(())
}


#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
