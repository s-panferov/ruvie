use crate::{component::ComponentExt, context::Render, html::el::div, prelude::*, Children};

fn button(ctx: &Render) -> Children {
	div::default().scope(|ctx| button.default()).into()
}

fn app(ctx: &Render) -> Children {
	button.default().into()
}
