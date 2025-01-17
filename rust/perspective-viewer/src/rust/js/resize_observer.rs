////////////////////////////////////////////////////////////////////////////////
//
// Copyright (c) 2018, the Perspective Authors.
//
// This file is part of the Perspective library, distributed under the terms
// of the Apache License 2.0.  The full license can be found in the LICENSE
// file.

// `rustfmt` removes `async` from extern blocks in rust stable
// [issue](https://github.com/rust-lang/rustfmt/issues/4288)

use wasm_bindgen::prelude::*;
// use web_sys::HtmlElement;

#[wasm_bindgen(inline_js = "export default ResizeObserver")]
extern "C" {
    pub type ResizeObserver;

    #[wasm_bindgen(constructor, js_class = "default")]
    pub fn new(callback: &js_sys::Function) -> ResizeObserver;

    #[wasm_bindgen(method)]
    pub fn observe(this: &ResizeObserver, elem: &web_sys::HtmlElement);

    #[wasm_bindgen(method)]
    pub fn unobserve(this: &ResizeObserver, elem: &web_sys::HtmlElement);

    pub type ResizeObserverEntry;

    #[wasm_bindgen(method, getter, js_name = "contentRect")]
    pub fn content_rect(this: &ResizeObserverEntry) -> web_sys::DomRectReadOnly;

}
