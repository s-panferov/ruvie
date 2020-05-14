use std::future::Future;
use std::task::Poll;
use std::{hash::Hash, rc::Rc};

use crate::{api, error::HashableError};
use observe::{
    local::{EvalContext, Value},
    Var,
};

use rustcss::{
    color::{BasicColor, Color},
    prelude::*,
    PositionType, StyleSheet,
};

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
    pub id: uuid::Uuid,
    pub title: String,
    pub completed: bool,
}

#[observe::store]
pub struct AppStore {
    pub props: AppProps,

    #[computed]
    pub style: Value<Option<StyleSheet>>,

    #[autorun(future = "wasm")]
    pub data: Value<Poll<Result<api::TestResponse, HashableError>>>,

    pub tasks: Value<Vec<Task>>,
}

impl AppStore {
    #[observe::constructor]
    pub fn new(props: AppProps) -> Rc<Self> {
        Rc::new(AppStore {
            props,
            style: Value::uninit(),
            data: Value::uninit(),
            tasks: Value::from(Var::new(vec![Task {
                id: uuid::Uuid::new_v4(),
                title: "Test".to_owned(),
                completed: false,
            }])),
        })
    }

    fn style(&self, ev: &mut EvalContext) -> Option<StyleSheet> {
        let AppProps { x, y, theme } = &self.props;

        let mut style = StyleSheet::new();

        style
            .height(100.px())
            .width(200.px())
            // .background_color(Rgba)
            .position(PositionType::Absolute);

        if *theme.observe(ev) == Theme::Square {
            style.background_color(Color::from(BasicColor::Green));
        } else {
            style.background_color(Color::from(BasicColor::Red));
        }

        style.left(x.observe(ev).px());
        style.top(y.observe(ev).px());
        Some(style)
    }

    fn data(
        &self,
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
