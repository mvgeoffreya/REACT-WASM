use std::fmt::Debug;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[derive(Debug)]
    type HTMLDocument;
    #[derive(Debug)]
    pub type Element;
    type HtmlCollection;
    static document: HTMLDocument;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;

    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    #[wasm_bindgen(method, js_name = appendChild)]
    fn append(this: &Element, item: Element);

    #[wasm_bindgen(method, setter = innerHTML)]
    fn add_value(this: &Element, html: &str);

    #[wasm_bindgen (structural , method , setter , js_class = "Element" , js_name = className)]
    fn class_name(this: &Element, value: &str);

    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen (method , structural , js_class = "Element" , js_name = getElementsByClassName)]
    pub fn get_elements_by_class_name(this: &Element, class_names: &str) -> HtmlCollection;
}

#[wasm_bindgen]
pub fn opening_alert(item: &str) {
    alert(&format!(
        "Welcome to Sample Basic Webiste with WASM !! my name is {}",
        item
    ));
}

#[wasm_bindgen]
pub fn generate(item: &str, class_name_tag: &Element) {
    class_name_tag.add_value(item);
}
