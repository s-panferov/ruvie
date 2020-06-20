use serde::Deserialize;
use std::fs::File;

use std::fmt::Write as FmtWrite;
use std::{
	collections::BTreeSet,
	io::{Error, Write},
};

#[derive(Debug, Deserialize)]
struct Types {
	version: f64,
	properties: Vec<Property>,
}

#[derive(Debug, Deserialize)]
struct Property {
	name: String,
	values: Option<Vec<Value>>,
	syntax: Option<String>,
	references: Option<Vec<Reference>>,
	description: Option<String>,
	browsers: Option<Vec<String>>,
	restrictions: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Value {
	name: String,
	description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Reference {
	name: String,
	url: String,
}

#[derive(Debug)]
pub struct Alternatives {
	variants: Vec<Syntax>,
}

#[derive(Debug)]
pub struct Options {
	variants: Vec<Syntax>,
}

#[derive(Debug)]
pub enum Type {
	Keyword(String),
	Spec(String),
	Quoted(String),
}

#[derive(Debug)]
pub struct Repetition {
	syntax: Syntax,
	min: usize,
	max: usize,
}

#[derive(Debug)]
pub enum Syntax {
	Alternatives(Box<Alternatives>),
	Options(Box<Options>),
	Repetition(Box<Repetition>),
	Type(Type),
}

// Example custom build script.
fn main() -> Result<(), Box<Error>> {
	println!("cargo:rerun-if-changed=types.json");

	let file = File::open("types.json")?;
	let json: Types = serde_json::from_reader(&file).unwrap();
	std::fs::create_dir_all("codegen")?;

	let mut mod_rs = std::fs::File::create("./codegen/mod.rs")?;

	writeln!(mod_rs, "{}", "mod kv;")?;

	let mut kv_rs = std::fs::File::create("./codegen/kv.rs")?;
	let mut kv = BTreeSet::new();

	for ty in json.properties.iter().take(20) {
		if ty.name.starts_with("::") {
			// pseudo-element
			continue;
		}

		let mod_name = inflector::cases::snakecase::to_snake_case(&ty.name);

		let mut file = std::fs::File::create(format!("./codegen/{}.rs", mod_name))?;
		writeln!(mod_rs, "pub mod {};", mod_name)?;

		let class_name = inflector::cases::classcase::to_class_case(&ty.name);
		let ident = quote::format_ident!("{}", class_name);
		let src = quote::quote! {
			struct #ident;
		};

		writeln!(file, "{}", src)?;

		if ty.syntax.is_some() {
			let syntax_str = ty.syntax.as_ref().unwrap();
			let res = syntax(&syntax_str);
			match res {
				Ok((_, syntax)) => {
					eprintln!("SYNTAX: {} \n {:#?}", syntax_str, syntax);
					let res = format_syntax(&ident, syntax, &mut kv)?;
					writeln!(file, "{}", res).unwrap()
				}
				Err(_) => eprintln!("Unknown syntax: {}", syntax_str),
			}
		}
	}

	for k in kv.iter() {
		let ident = quote::format_ident!("{}", k);
		let src = quote::quote! {
			struct #ident;
			impl std::fmt::Display for #ident {
				fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
					write!(f, #k);
				}
			}
		};

		writeln!(kv_rs, "{}", src)?;
	}

	Ok(())
}

fn format_syntax(
	cls: &proc_macro2::Ident,
	syn: Syntax,
	kv: &mut BTreeSet<String>,
) -> Result<String, Box<Error>> {
	let mut res = String::new();
	match syn {
		Syntax::Type(t) => match t {
			Type::Keyword(k) => match k.as_ref() {
				"auto" => writeln!(res, "impl ValueFor<{}> for ruvie_css::kv::Auto;", cls).unwrap(),
				_ => {
					let keyword = inflector::cases::classcase::to_class_case(&k);
					let keyword = quote::format_ident!("{}", keyword);

					writeln!(
						res,
						"impl ValueFor<{}> for ruvie_css::kv::{};",
						cls, keyword
					)
					.unwrap();

					kv.insert(k);
				}
			},
			Type::Spec(s) => match s.as_ref() {
				"length" => writeln!(res, "impl ValueFor<{}> for ruvie_css::Length;", cls).unwrap(),
				"percentage" => {
					writeln!(res, "impl ValueFor<{}> for ruvie_css::Percentage;", cls).unwrap()
				}
				_ => {}
			},
			_ => {}
		},
		Syntax::Alternatives(v) => {
			for alt in v.variants {
				writeln!(res, "{}", format_syntax(cls, alt, kv)?).unwrap();
			}
		}
		_ => {}
	};

	Ok(res)
}

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::{
	error::{ErrorKind, ParseError},
	multi::separated_list1,
	AsChar, IResult, InputTakeAtPosition,
};

fn syntax(input: &str) -> nom::IResult<&str, Syntax> {
	let (input, syn) = nom::branch::alt((alt, opts, repetition, group, ty))(input)?;
	Ok((input, syn))
}

fn alt(input: &str) -> nom::IResult<&str, Syntax> {
	let (input, left) = nom::branch::alt((repetition, group, ty))(input)?;
	let (input, _) = tag(" | ")(input)?;
	let (input, mut rest) =
		separated_list1(tag(" | "), nom::branch::alt((repetition, group, ty)))(input)?;

	let mut res = Vec::new();
	res.push(left);
	res.append(&mut rest);

	Ok((
		input,
		Syntax::Alternatives(Box::new(Alternatives { variants: res })),
	))
}

fn opts(input: &str) -> nom::IResult<&str, Syntax> {
	let (input, left) = nom::branch::alt((repetition, group, ty))(input)?;
	let (input, _) = tag(" || ")(input)?;
	let (input, mut rest) =
		separated_list1(tag(" || "), nom::branch::alt((repetition, group, ty)))(input)?;

	let mut res = Vec::new();
	res.push(left);
	res.append(&mut rest);

	Ok((input, Syntax::Options(Box::new(Options { variants: res }))))
}

fn ty(input: &str) -> nom::IResult<&str, Syntax> {
	let (input, ty) = nom::branch::alt((quoted, spec, keyword))(input)?;
	Ok((input, Syntax::Type(ty)))
}

fn quoted(input: &str) -> nom::IResult<&str, Type> {
	let (input, _) = tag("<'")(input)?;
	let (input, value) = name(input)?;
	let (input, _) = tag("'>")(input)?;

	Ok((input, Type::Quoted(value.to_string())))
}

fn group(input: &str) -> nom::IResult<&str, Syntax> {
	let (input, _) = tag("[")(input)?;
	let (input, _) = multispace0(input)?;
	let (input, value) = syntax(input)?;
	let (input, _) = multispace0(input)?;
	let (input, _) = tag("]")(input)?;
	Ok((input, value))
}

fn repetition(input: &str) -> nom::IResult<&str, Syntax> {
	let (input, syntax) = nom::branch::alt((group, ty))(input)?;
	let (input, _) = tag("{")(input)?;
	let (input, min) = digit1(input)?;
	let (input, _) = tag(",")(input)?;
	let (input, max) = digit1(input)?;
	let (input, _) = tag("}")(input)?;
	Ok((
		input,
		Syntax::Repetition(Box::new(Repetition {
			min: min.parse().unwrap(),
			max: max.parse().unwrap(),
			syntax,
		})),
	))
}

fn spec(input: &str) -> nom::IResult<&str, Type> {
	let (input, _) = tag("<")(input)?;
	let (input, value) = name(input)?;
	let (input, _) = tag(">")(input)?;

	Ok((input, Type::Spec(value.to_string())))
}

fn keyword(input: &str) -> nom::IResult<&str, Type> {
	let (input, value) = name(input)?;
	Ok((input, Type::Keyword(value.to_string())))
}

pub fn name<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
	T: InputTakeAtPosition,
	<T as InputTakeAtPosition>::Item: AsChar,
{
	input.split_at_position1_complete(
		|item| {
			let char = item.as_char();
			char == '>' || char == '\'' || char == ' ' || char == '['
		},
		ErrorKind::AlphaNumeric,
	)
}
