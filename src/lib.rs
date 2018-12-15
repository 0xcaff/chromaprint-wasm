use chromaprint::Fingerprinter;
use wasm_bindgen::prelude::*;
use wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct ChromaprintContext {
    inner: Fingerprinter,
}

#[wasm_bindgen]
impl ChromaprintContext {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChromaprintContext {
        ChromaprintContext {
            inner: Fingerprinter::new(44100),
        }
    }

    pub fn feed(&mut self, data: &[i16]) {
        self.inner.feed(data);
    }

    pub fn finish(mut self) -> String {
        self.inner.finish();
        self.inner.fingerprint().compress().encode()
    }
}
