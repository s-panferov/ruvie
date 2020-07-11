use html5ever::{namespace_url, ns, parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use std::io::Read;
use std::iter::repeat;

pub struct Template {}

fn walk(indent: usize, handle: &Handle) {
	let node = handle;
	// FIXME: don't allocate
	print!("{}", repeat(" ").take(indent).collect::<String>());
	match node.data {
		NodeData::Document => println!("#Document"),

		NodeData::Doctype {
			ref name,
			ref public_id,
			ref system_id,
		} => println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),

		NodeData::Text { ref contents } => {
			println!("#text: {}", escape_default(&contents.borrow()))
		}

		NodeData::Comment { ref contents } => println!("<!-- {} -->", escape_default(contents)),

		NodeData::Element {
			ref name,
			ref attrs,
			..
		} => {
			assert!(name.ns == ns!(html));
			print!("<{}", name.local);
			for attr in attrs.borrow().iter() {
				assert!(attr.name.ns == ns!());
				print!(" {}=\"{}\"", attr.name.local, attr.value);
			}
			println!(">");
		}

		NodeData::ProcessingInstruction { .. } => unreachable!(),
	}

	for child in node.children.borrow().iter() {
		walk(indent + 4, child);
	}
}

pub fn escape_default(s: &str) -> String {
	s.chars().flat_map(|c| c.escape_default()).collect()
}

fn parse<R: Read>(read: &mut R) {
	let dom = parse_document(RcDom::default(), Default::default())
		.from_utf8()
		.read_from(read)
		.unwrap();

	walk(0, &dom.document);

	if !dom.errors.is_empty() {
		println!("\nParse errors:");
		for err in dom.errors.iter() {
			println!("    {}", err);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::parse;

	#[test]
	fn test() {
		parse(&mut std::io::Cursor::new(
			String::from(r#"<template id="test"> asdf</template>"#).as_bytes(),
		));
	}
}
