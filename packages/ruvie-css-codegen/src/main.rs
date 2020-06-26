use std::fs::File;

use std::fmt::Write as FmtWrite;
use std::{
	collections::{BTreeMap, BTreeSet},
	io::{Error, Write},
	process::{Child, Command, Stdio},
};

use inflector::cases::classcase::to_class_case;
use inflector::cases::snakecase::to_snake_case;
use types::{Syntax, Type};

mod types;

// Example custom build script.
fn main() -> Result<(), Box<Error>> {
	let file = File::open("./data/css/properties.json")?;

	let json: BTreeMap<String, types::Property> = serde_json::from_reader(&file).unwrap();
	std::fs::create_dir_all("../ruvie-css/src/props")?;

	let mut mod_rs = std::fs::File::create("../ruvie-css/src/props/mod.rs")?;

	for (name, ty) in json
		.iter()
		.filter(|(name, _)| !name.starts_with("::") && !name.starts_with("-"))
	{
		let mod_name = to_snake_case(&name);
		let mut file = std::fs::File::create(format!("../ruvie-css/src/props/{}.rs", mod_name))?;

		writeln!(
			mod_rs,
			"pub mod {module}; pub use {module}::*;",
			module = mod_name
		)?;

		let class_name = to_class_case(&name);
		let ident = quote::format_ident!("{}", class_name);
		let func_name = quote::format_ident!("{}", mod_name);

		let mut kv = BTreeSet::new();
		let syntax_str = &ty.syntax;
		let res = crate::syntax::SyntaxParser::new().parse(&syntax_str);

		println!("PROP {} HAS SYNTAX: {} \n {:#?}", name, syntax_str, res);
		let source = match res {
			Ok(syntax) => Some(format_syntax(&ident, syntax, &mut kv)?),
			Err(_) => None,
		};

		let keywords = kv
			.iter()
			.map(|k| quote::format_ident!("{}", to_class_case(k)))
			.collect::<Vec<_>>();

		let formatting = kv
			.iter()
			.map(|k| {
				let name = quote::format_ident!("{}", to_class_case(k));
				quote::quote! {
					#ident::#name => write!(f, #k)
				}
			})
			.collect::<Vec<_>>();

		let display = if kv.len() > 0 {
			quote::quote! {
				impl std::fmt::Display for #ident {
					fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
							match self {
								#(
									#formatting,
								)*
							}
					}
				}

				impl crate::ValueFor<#ident> for #ident {}
			}
		} else {
			quote::quote! {}
		};

		let src = quote::quote! {
			pub enum #ident {
				#(
					#keywords
				),*
			}

			#display

			impl crate::Attribute for #ident {
				const NAME: &'static str = #name;
			}

			impl crate::StyleSheet {
				pub fn #func_name<V: crate::ValueFor<#ident>>(mut self, value: V) -> Self {
					self.rules.insert(#name, value.value());
					self
				}
			}
		};

		writeln!(file, "{}", src)?;

		if let Some(source) = source {
			writeln!(file, "{}", source)?;
		}
	}

	format().unwrap();

	Ok(())
}

fn format() -> std::io::Result<Child> {
	Command::new("cargo")
		.arg("fmt")
		.current_dir("../ruvie-css")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.stderr(Stdio::null())
		.spawn()
}

fn format_syntax(
	cls: &proc_macro2::Ident,
	syn: Syntax,
	kv: &mut BTreeSet<String>,
) -> Result<String, Box<Error>> {
	let mut res = String::new();
	match syn {
		Syntax::Alt(v) => {
			for alt in v.variants {
				writeln!(res, "{}", format_syntax(cls, alt, kv)?).unwrap();
			}
		}
		Syntax::Type(t) => match t {
			Type::Keyword(k) => match k {
				_ => {
					kv.insert(k);
				}
			},
			Type::Spec(s) => match s.as_ref() {
				"length" => writeln!(
					res,
					"impl crate::ValueFor<{}> for crate::types::length::Length {{}}",
					cls
				)
				.unwrap(),
				"integer" => writeln!(
					res,
					"
					impl crate::ValueFor<{cls}> for usize {{}}
					impl crate::ValueFor<{cls}> for isize {{}}
					",
					cls = cls
				)
				.unwrap(),
				"number" => writeln!(
					res,
					"
					impl crate::ValueFor<{cls}> for usize {{}}
					impl crate::ValueFor<{cls}> for isize {{}}
					impl crate::ValueFor<{cls}> for f32 {{}}
					impl crate::ValueFor<{cls}> for f64 {{}}
					",
					cls = cls
				)
				.unwrap(),
				"alpha-value" => writeln!(
					res,
					"
					impl crate::ValueFor<{cls}> for f32 {{}}
					impl crate::ValueFor<{cls}> for f64 {{}}
					",
					cls = cls
				)
				.unwrap(),
				"color" => writeln!(
					res,
					"
					impl crate::ValueFor<{cls}> for crate::types::color::Color {{}}
					",
					cls = cls
				)
				.unwrap(),
				"percentage" => writeln!(
					res,
					"impl crate::ValueFor<{}> for crate::types::percentage::Percentage {{}}",
					cls
				)
				.unwrap(),
				"length-percentage" => writeln!(
					res,
					r#"
					impl crate::ValueFor<{cls}> for crate::types::length::Length {{}}
					impl crate::ValueFor<{cls}> for crate::types::percentage::Percentage {{}}
					"#,
					cls = cls
				)
				.unwrap(),
				_ => eprintln!("SPEC TYPE {} IS UNKNOWN", s),
			},
			_ => {}
		},

		_ => {}
	};

	Ok(res)
}

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub syntax); // synthesized by LALRPOP
