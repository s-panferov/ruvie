use crate::{
	component::ComponentExt,
	context::Render,
	html::{el::div, props::Class},
	prelude::*,
	Children,
};

use observe::Value;

fn button(ctx: &Render) -> Children {
	div::prop(Class(Value::from(crate::cx!("test")))).into()
}

fn app(ctx: &Render) -> Children {
	button.default().into()
}
