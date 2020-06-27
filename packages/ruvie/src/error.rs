use wasm_bindgen::JsValue;

#[derive(thiserror::Error, Debug)]
pub enum RuvieError {
	#[error("Error: {}", .0)]
	Other(#[source] anyhow::Error),

	#[error("JsError")]
	JsError(JsValue),
}

impl From<JsValue> for RuvieError {
	fn from(value: JsValue) -> Self {
		RuvieError::JsError(value)
	}
}
