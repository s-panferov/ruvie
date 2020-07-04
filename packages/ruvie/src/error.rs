#[derive(thiserror::Error, Debug)]
pub enum RuvieError {
	#[error("Error: {}", .0)]
	Other(#[source] anyhow::Error),

	#[cfg(feature = "web")]
	#[error("JsError")]
	JsError(wasm_bindgen::JsValue),
}

#[cfg(feature = "web")]
impl From<wasm_bindgen::JsValue> for RuvieError {
	fn from(value: wasm_bindgen::JsValue) -> Self {
		RuvieError::JsError(value)
	}
}
