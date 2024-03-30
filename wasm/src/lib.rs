extern crate console_error_panic_hook;
extern crate web_sys;

mod definitions;
mod error;
mod utils;
mod workers;

use definitions::generate_typescript_definitions;
use std::panic;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use workers::parse_toml;

/// Generator for creating TypeScript type definitions based off of Cloudflare Worker Configs.
#[wasm_bindgen]
pub struct WorkerBindingGenerator {
    previous_definitions: String,
}

#[wasm_bindgen]
impl WorkerBindingGenerator {
    /// Takes the wrangler `config_path` (traditionally `wrangler.toml``) and extracts the type
    /// definitions of all the bindings into a `.d.ts` file in the `output_path` location.
    pub fn generate_definitions(
        &mut self,
        contents: String,
    ) -> Result<GenerateDefinitionsResponse, JsValue> {
        let config = parse_toml(&contents).map_err(JsValue::from)?;
        let definitions = generate_typescript_definitions(&config);
        let definitions_str = definitions.to_string();
        let should_update = definitions_str != self.previous_definitions;
        if should_update {
            self.previous_definitions = definitions_str;
        }
        Ok(GenerateDefinitionsResponse {
            definitions: self.previous_definitions.to_string(),
            should_update,
        })
    }

    #[wasm_bindgen(constructor)]
    pub fn new() -> WorkerBindingGenerator {
        WorkerBindingGenerator {
            previous_definitions: String::new(),
        }
    }
}

#[wasm_bindgen]
pub struct GenerateDefinitionsResponse {
    definitions: String,
    should_update: bool,
}

#[wasm_bindgen]
impl GenerateDefinitionsResponse {
    /// The string version of the typedef file.
    pub fn definitions(&self) -> String {
        self.definitions.to_owned()
    }

    /// Whether the definitions have changed and should be updated.
    ///
    /// This is set when calling the generate_definitions function and applies
    /// to the previous call to the function.
    pub fn should_update(&self) -> bool {
        self.should_update
    }
}
