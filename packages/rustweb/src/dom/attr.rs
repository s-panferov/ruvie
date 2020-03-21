use observe::MaybeValue;
use rustcss::StyleSheet;
use std::fmt::Display;

#[derive(Debug, Default)]
pub struct DefaultAttributes {
    pub style: MaybeValue<StyleSheet>,
    pub class: MaybeValue<ClassList>,
    pub contenteditable: MaybeValue<bool>,
}

pub trait Attribute {
    fn name(&self) -> &str;
}

#[derive(Debug, Hash)]
pub struct ClassList {
    pub classes: Vec<String>,
}

impl Display for ClassList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cls in &self.classes {
            write!(f, ".{}", cls)?
        }
        Ok(())
    }
}

impl Attribute for ClassList {
    fn name(&self) -> &str {
        "class"
    }
}
