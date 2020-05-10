#![allow(non_snake_case)]

use std::future::Future;
use std::task::Poll;
use std::{fmt::Display, hash::Hash, rc::Rc};

use wasm_bindgen::{prelude::*, JsCast, JsValue};

use observe::{
    local::{EvalContext, Value},
    transaction, Computed, Var,
};

use rustcss::{
    color::{BasicColor, Color},
    prelude::*,
    PositionType, StyleSheet,
};

use rustweb::{
    context::Render,
    dom::{el::div, Class, ClassList, Html, Style},
    prelude::*,
    Children,
};

mod api;

#[derive(Hash, PartialEq, Debug)]
enum Theme {
    Square,
    Circle,
}

#[derive(Debug)]
struct AppProps {
    theme: Value<Theme>,
    x: Value<i32>,
    y: Value<i32>,
}

fn Button(ctx: &mut Render<(), Html>) -> Children<Html> {
    div().default().children(ctx.children.clone()).into()
}

#[derive(Debug)]
pub struct HashableError(anyhow::Error);

impl Display for HashableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<E> From<E> for HashableError
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(e: E) -> Self {
        HashableError(e.into())
    }
}

impl Hash for HashableError {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{:#?}", self.0).hash(state);
    }
}

#[observe::store]
struct AppStore {
    props: AppProps,

    #[computed]
    style: Value<Option<StyleSheet>>,

    #[autorun(future = "wasm")]
    data: Value<Poll<Result<api::TestResponse, HashableError>>>,
}

fn location() -> url::Url {
    let location = web_sys::window().unwrap().location().href().expect("Href");
    url::Url::parse(&location).unwrap()
}

impl AppStore {
    #[observe::constructor]
    fn new(props: AppProps) -> Rc<Self> {
        Rc::new(AppStore {
            props,
            style: Value::uninit(),
            data: Value::uninit(),
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

fn App(ctx: &mut Render<AppStore, Html>) -> Children<Html> {
    let store = ctx.props;
    let _ = store.props.theme.observe(&mut ctx.eval);

    let payload = store.data.clone();

    div()
        .prop(Style, store.style.clone())
        .prop(Class, ClassList::new(vec!["test".to_owned()]))
        .scope(move |_ctx| {
            let payload = payload.clone();
            Value::from(Computed::new(move |eval| match &*payload.observe(eval) {
                Poll::Ready(_v) => format!("DATA: {:?}", _v),
                Poll::Pending => String::from("Loading"),
            }))
        })
        // .child("Test")
        // .child(Button.create().default().child("Test"))
        // .scope(move |_| "Test")
        .into()
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let x: Value<_> = 0.into();
    let y: Value<_> = 0.into();
    let theme = Value::from(Var::new(Theme::Square));

    let store = AppStore::new(AppProps {
        theme: theme.clone(),
        x: x.clone(),
        y: y.clone(),
    });

    let app = App.create().with_props(store).build();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let mousemove = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        transaction(None, |tx| {
            x.set(tx, event.x());
            y.set(tx, event.y());
        });
    }) as Box<dyn FnMut(_)>);

    let click = Closure::wrap(Box::new(move |_ev: web_sys::MouseEvent| {
        transaction(None, |tx| {
            let current = theme.once();
            theme.set(
                tx,
                match *current {
                    Theme::Circle => Theme::Square,
                    Theme::Square => Theme::Circle,
                },
            );
        });
    }) as Box<dyn FnMut(_)>);

    document.add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("click", click.as_ref().unchecked_ref())?;

    mousemove.forget();
    click.forget();

    let (node, instance) = rustweb::render(Rc::new(app), None)?;
    body.append_child(&node)?;

    Box::leak(Box::new(instance));

    Ok(())
}
