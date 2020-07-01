use crate::{component::ComponentExt, context::Render, prelude::*, web::elem::div, Children};

fn button(ctx: &Render) -> Children {
	div::default().scope(|ctx| button.default()).into()
}

fn app(ctx: &Render) -> Children {
	button.default().into()
}
