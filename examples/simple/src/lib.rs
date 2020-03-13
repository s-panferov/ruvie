#![allow(non_snake_case)]

use rustweb::dom::Text;
use std::sync::Arc;

use observe::transaction;
use observe::EvalContext;
use rustweb::dom::HtmlProps;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use observe::{Computed, Value, Var};
use rustweb::{
    dom::{div, Html},
    prelude::*,
    Children, Context,
};

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

fn Button(ctx: Context<()>) -> Children<Html> {
    let Context { children, .. } = ctx;
    div().default().children(children.clone()).into()
}

#[observe::store]
struct AppStore {
    props: AppProps,
    style: Computed<String>,
}

impl AppStore {
    #[observe::create]
    fn new(props: AppProps) -> Arc<Self> {
        Arc::new(AppStore {
            props,
            style: Default::default(),
        })
    }

    fn style(&self, ev: &mut EvalContext) -> String {
        let AppProps { x, y, theme } = &self.props;

        let mut style = "height: 100px; position: absolute; width: 100px;".to_owned();
        if *theme.observe(ev) == Theme::Square {
            style += "background-color: green;"
        } else {
            style += "background-color: red;"
        }

        style += &format!("left: {}px;", &x.observe(ev));
        style += &format!("top: {}px;", &y.observe(ev));
        style
    }
}

fn App(ctx: Context<Arc<AppStore>>) -> Children<Html> {
    let Context { props, .. } = ctx;

    let _ = props.props.theme.observe(ctx.eval);

    div()
        .with({
            let style = props.style.clone();
            HtmlProps {
                attributes: Computed::new(move |ctx| {
                    maplit::btreemap! {
                        "style".to_owned() => style.observe(ctx).to_string()
                    }
                })
                .into(),
            }
        })
        .child("Test")
        .child(Button.create().default())
        .scope(move |_| "Test")
        .into()
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let x = Var::new(0);
    let y = Var::new(0);
    let theme = Var::new(Theme::Square);

    let store = AppStore::new(AppProps {
        theme: theme.clone().into(),
        x: x.clone().into(),
        y: y.clone().into(),
    });

    let app = App.create().with(store);

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let mousemove = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        transaction(None, |tx| {
            x.set(event.x(), tx);
            y.set(event.y(), tx);
        });
    }) as Box<dyn FnMut(_)>);

    let click = Closure::wrap(Box::new(move |_ev: web_sys::MouseEvent| {
        transaction(None, |tx| {
            let current = theme.once();
            theme.set(
                match *current {
                    Theme::Circle => Theme::Square,
                    Theme::Square => Theme::Circle,
                },
                tx,
            );
        });
    }) as Box<dyn FnMut(_)>);

    document.add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("click", click.as_ref().unchecked_ref())?;

    mousemove.forget();
    click.forget();

    let instance = rustweb::dom::render(body, app)?;
    Box::leak(Box::new(instance));

    Ok(())
}
