use std::future::Future;
use std::{hash::Hash, sync::Arc};

use crate::{api, error::HashableError};
use observe::{
	future::{ComputedFuture, WasmBindgen},
	Computed, Observable, Var, {EvalContext, Value},
};

use ruvie_css::{
	color::{BasicColor, Color},
	prelude::*,
	PositionType, StyleSheet,
};

use ruvie::contrib::list::IndexList;

#[derive(Hash, PartialEq, Debug)]
pub enum Theme {
	Square,
	Circle,
}

#[derive(Debug)]
pub struct AppProps {
	pub theme: Value<Theme>,
	pub x: Value<i32>,
	pub y: Value<i32>,
}

fn location() -> url::Url {
	let location = web_sys::window().unwrap().location().href().expect("Href");
	url::Url::parse(&location).unwrap()
}

#[derive(Hash, Clone)]
pub struct Task {
	pub id: usize,
	pub title: String,
	pub completed: bool,
}

pub struct AppStore {
	pub props: AppProps,
	pub style: Computed<Option<Arc<StyleSheet>>>,
	pub data: ComputedFuture<Result<api::TestResponse, HashableError>, WasmBindgen>,
	pub tasks: Var<Arc<IndexList<usize, Task>>>,
}

impl AppStore {
	pub fn new(props: AppProps) -> Arc<Self> {
		let AppProps { x, y, theme } = &props;
		let style = observe::computed!((x, y, theme) ctx => Self::style(&x, &y, &theme, ctx));
		let map = indexmap::indexmap! {
			0 => Task {
				id: 0,
				title: "Test".to_owned(),
				completed: false,
			}
		};

		Arc::new(AppStore {
			props,
			style,
			data: observe::future!(() ctx => Self::data(ctx)),
			tasks: Var::new(Arc::new(map.into())),
		})
	}

	fn style(
		x: &Value<i32>,
		y: &Value<i32>,
		theme: &Value<Theme>,
		ev: &mut EvalContext,
	) -> Option<Arc<StyleSheet>> {
		let mut style = StyleSheet::new();

		style
			.height(100.px())
			.width(200.px())
			// .background_color(Rgba)
			.position(PositionType::Absolute);

		if *theme.get(ev) == Theme::Square {
			style.background_color(Color::from(BasicColor::Green));
		} else {
			style.background_color(Color::from(BasicColor::Red));
		}

		style.left(x.get(ev).px());
		style.top(y.get(ev).px());
		Some(Arc::new(style))
	}

	fn data(
		_ev: &mut EvalContext,
	) -> impl Future<Output = Result<api::TestResponse, HashableError>> {
		async {
			let mut url = location();
			url.set_path("/api/test");
			let resp = reqwest::get(url).await?.json::<api::TestResponse>().await?;
			Ok(resp)
		}
	}
}
