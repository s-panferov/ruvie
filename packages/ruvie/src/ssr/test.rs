use crate::{
	component::ComponentExt,
	context::Render,
	html::{el::div, props::Class},
	prelude::*,
	Children,
};

use observe::Value;

fn button(ctx: &Render) -> Children {
	let sheet = StyleSheet::new().background_color(Raw(String::from("red")));
	div::prop(Class(Value::from(crate::cx!["test", sheet]))).into()
}

fn app(ctx: &Render) -> Children {
	vec![
		button.default().build(),
		button.default().build(),
		button.default().build(),
	]
	.into()
}

use crate::{ssr::Static, Runtime};
use ruvie_css::{Raw, StyleSheet};

#[test]
fn test() {
	let rt = Runtime::new(Static::new());
	let view = rt.render(app.default(), Box::new(())).unwrap();
	rt.tick().unwrap();

	// assert_eq!(
	// 	crate::ssr::stringify(&view),
	// 	String::from(
	// 		r#"
	// 		<!--<Function>-->
	// 			<div class="test"></div>
	// 			<div class="test"></div>
	// 			<div class="test"></div>
	// 		<!--</Function>-->
	// 		"#
	// 	)
	// 	.trim()
	// 	.replace("\n", "")
	// 	.replace("\r", "")
	// 	.replace("\t", "")
	// );

	let target = rt.platform.downcast_ref::<Static>().unwrap();
	let styles = target.styles.serialize();

	assert_eq!(styles, "")
}
