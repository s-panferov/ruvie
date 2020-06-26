use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Property {
	pub syntax: String,
}

// #[derive(Debug, Deserialize)]
// struct Value {
// 	name: String,
// 	description: Option<String>,
// }

// #[derive(Debug, Deserialize)]
// struct Reference {
// 	name: String,
// 	url: String,
// }

#[derive(Debug)]
pub struct Alt {
	pub variants: Vec<Syntax>,
}

#[derive(Debug)]
pub struct Options {
	pub variants: Vec<Syntax>,
}

#[derive(Debug)]
pub enum Type {
	Keyword(String),
	Spec(String),
	Reference(String),
	Parametrized(String, Box<Type>),
}

#[derive(Debug)]
pub struct Repetition {
	pub syntax: Syntax,
	pub min: u32,
	pub max: u32,
}

#[derive(Debug)]
pub enum Syntax {
	Alt(Box<Alt>),
	Options(Box<Options>),
	Repetition(Box<Repetition>),
	Type(Type),
}
