use crate::{component::ComponentExt, context::Render, prelude::*, web::elem::Div, Children};

fn button(ctx: &Render) -> Children {
	Div::default().scope(|ctx| button.default()).into()
}

fn app(ctx: &Render) -> Children {
	button.default().into()
}
