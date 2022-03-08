use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" { 
    #[wasm_bindgen(js_namespace=htmx)]
    pub fn process(el: &web_sys::Element); 

    #[wasm_bindgen(js_namespace=htmx)]
    pub fn remove(el: web_sys::Element); 

    #[wasm_bindgen(js_namespace=htmx)]
    pub fn find(selector: &str) -> web_sys::Element; 
}