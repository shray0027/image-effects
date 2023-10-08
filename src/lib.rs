use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use  base64::{decode,encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn grayscale(encoded_file : &str) -> String{
    log(&"Grayscale called".into());
    let base64_to_vec = decode(encoded_file).unwrap();
    log(&"image uploaded".into());
    let mut img = load_from_memory(&base64_to_vec).unwrap();
    log(&"image loaded".into());
    img = img.grayscale();
    log(&"grayscale effect applied".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer,Png).unwrap();
    log(&"new image written".into());

    let encoded_img = encode(&buffer);
    let data_url = format!(
        "data:image/png;base64,{}",
        encoded_img
    );
    data_url
}