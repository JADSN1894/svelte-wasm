use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

cfg_if::cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    }
}

cfg_if::cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

macro_rules! log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug, Deserialize, Tsify)]
#[tsify(namespace)]
#[serde(tag = "cmd", content = "data", rename_all = "lowercase")]
pub enum Cmd {
    Md5(String),
    Sha1(String),
    Sha256(String),
    Sha512(String),
    Nil,
}

#[wasm_bindgen]
pub fn bip_0039_mnemonic_phrase() -> String {
    use bip0039::{Count, English, Mnemonic};

    // Generates an English mnemonic with 12 words randomly
    let mnemonic = <Mnemonic<English>>::generate(Count::Words12);

    // Gets the phrase
    let phrase = mnemonic.phrase();

    phrase.to_string()
}

#[wasm_bindgen]
pub fn dispatch(val: &str) -> String {
    log!("{val}");
    match serde_json::from_str::<Cmd>(val) {
        Ok(cmd) => match cmd {
            Cmd::Md5(data) => {
                use md5::{Digest, Md5};

                // * Create a Md5 object
                let mut hasher = Md5::new();

                // * Write input message
                hasher.update(data.as_bytes());

                // * Read hash digest and consume hasher
                let hash = hasher.finalize();
                let mut result = "".to_string();

                for byte in hash {
                    let serialized_byte = format!("{:02x}", byte);
                    result.push_str(&serialized_byte)
                }

                result
            }
            Cmd::Sha1(data) => {
                use sha1::{Digest, Sha1};

                // * Create a Md5 object
                let mut hasher = Sha1::new();

                // * Write input message
                hasher.update(data.as_bytes());

                // * Read hash digest and consume hasher
                let hash = hasher.finalize();
                let mut result = "".to_string();

                for byte in hash {
                    let serialized_byte = format!("{:02x}", byte);
                    result.push_str(&serialized_byte)
                }

                result
            }
            Cmd::Sha256(data) => {
                use sha2::{Digest, Sha256};

                // * Create a Md5 object
                let mut hasher = Sha256::new();

                // * Write input message
                hasher.update(data.as_bytes());

                // * Read hash digest and consume hasher
                let hash = hasher.finalize();
                let mut result = "".to_string();

                for byte in hash {
                    let serialized_byte = format!("{:02x}", byte);
                    result.push_str(&serialized_byte)
                }

                result
            }
            Cmd::Sha512(data) => {
                use sha2::{Digest, Sha512};

                // * Create a Md5 object
                let mut hasher = Sha512::new();

                // * Write input message
                hasher.update(data.as_bytes());

                // * Read hash digest and consume hasher
                let hash = hasher.finalize();
                let mut result = "".to_string();

                for byte in hash {
                    let serialized_byte = format!("{:02x}", byte);
                    result.push_str(&serialized_byte)
                }

                result
            }
            Cmd::Nil => "Nil".into(),
        },
        Err(error) => {
            log!("[ERROR] {:?}", error);
            "Posible values of encode are: Md5, SHA1 Sha256, Sha512".into()
        }
    }

    // dbg!(encode_cmd);
}
