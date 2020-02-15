use rustweb::{div, render, Component, Html, HttpProps};

struct App {}

impl Component for App {
    fn render(&self) -> Html {
        div(Props, || vec![div(Props, || {})])
    }
}

fn main() {
    stdweb::initialize();
    stdweb::console!(log, html("test".to_owned()));
    render(stdweb::web::document().body().unwrap());
    stdweb::event_loop();
}
