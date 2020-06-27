use crate::{
	component::ComponentExt, context::Render, scope::Scope, web::elem::Div, Children, Component,
};

fn button(ctx: Render) -> Children {
	Div::default().into()
}

fn render() {}
