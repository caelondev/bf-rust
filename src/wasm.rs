use wasm_bindgen::prelude::*;

use crate::BrainfuckRust;

#[wasm_bindgen]
unsafe extern "C" {
    #[wasm_bindgen(js_namespace = globalThis, js_name = bfWrite)]
    unsafe fn bf_write(s: &str);

    #[wasm_bindgen(js_namespace = globalThis, js_name = bfRead)]
    unsafe fn bf_read() -> String;
}

#[wasm_bindgen]
pub struct Brainfuck {
    bf: BrainfuckRust,
}

#[wasm_bindgen]
impl Brainfuck {
    #[wasm_bindgen(constructor)]
    pub fn new(source: &str) -> Self {
        Self {
            bf: BrainfuckRust::with_io(
                source,
                |ch| {
                    let mut s = String::new();
                    s.push(ch);
                    unsafe {
                        bf_write(&s);
                    }
                },
                || unsafe { bf_read().bytes().next().unwrap_or(0) },
            ),
        }
    }

    pub fn compile(&mut self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.bf.compile()).map_err(|e| e.to_string().into())
    }

    pub fn run(&mut self, instructions: JsValue) -> Result<(), String> {
        let instructions: Vec<(char, u8)> =
            serde_wasm_bindgen::from_value(instructions).map_err(|e| e.to_string())?;
        self.bf.run(instructions)
    }
}
