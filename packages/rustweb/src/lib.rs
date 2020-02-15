extern crate web_sys;

pub enum Change {
    CreateElement { name: &'static str },
    SetAttribute { name: &'static str, value: String },
}

pub trait Component: Sized {
    type Props;
    fn render(&self) -> Html;
    fn create(self, props: Self::Props) -> Element<Self> {
        Element {
            spec: self,
            props,
            dom_node: None,
        }
    }
}

pub struct Element<C: Component> {
    spec: C,
    props: C::Props,
    dom_node: Option<web_sys::HtmlElement>,
}

mod native {
    use super::{Change, Component, Html};
    pub struct Div {}

    #[derive(Default)]
    pub struct HttpProps {
        style: Option<String>,
    }

    impl Component for Div {
        type Props = HttpProps;
        fn render(&self) -> Html {
            return vec![Change::CreateElement { name: "div" }];
        }
    }
}

mod el {
    use super::native::{Div, HttpProps};
    use super::{Component, Html};
    pub fn div(props: HttpProps) -> Html {
        Div {}.create(props)
    }
}

pub type Html = Vec<Element>;

pub fn render<C: Component>(el: web_sys::HtmlElement, component: C, props: C::Props) {}
