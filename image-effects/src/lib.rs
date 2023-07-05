use base64::{decode, encode};
use image::{load_from_memory, ImageOutputFormat};
use wasm_bindgen::prelude::wasm_bindgen;

// NOTE Helper macro to call console.log
/// Borrows the expression and converts it to a `JsValue` to call `web_sys::console::log_1`
/// # Example
/// ```rust
/// js_log!("Hello World") // log(&"Hello World".into());
/// ```
macro_rules! js_log {
    ($value:expr) => {
        // NOTE Importing JS functions from web_sys
        ::web_sys::console::log_1(&$value.into());
    };
}

#[wasm_bindgen]
pub fn grayscale(file_data: &str) -> String {
    js_log!("Grayscale called");

    let file_data: Vec<u8> = decode(file_data).expect("Error decoding file to base64!");
    js_log!("Image decoded");

    let img = load_from_memory(&file_data).expect("Error loading image from memory!");
    js_log!("Image loaded");

    let img = img.grayscale();
    js_log!("Grayscale effect applied");

    let mut buffer = vec![];
    img.write_to(&mut buffer, ImageOutputFormat::Png)
        .expect("Error writing image to buffer!");
    js_log!("New image created");

    let img = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", img);
    js_log!("New image encoded");

    data_url
}
