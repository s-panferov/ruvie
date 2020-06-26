// pub use enclose::*;

#[macro_export]
macro_rules! style2 {
	($name:ident, |$ctx:ident| $b:block) => {
		lazy_static::lazy_static!(static ref $name: StyleSheet = {
            let mut style = ruvie::StyleSheet::new();
            style.with_name(std::stringify!($name));
            let constr = move |$ctx: &mut StyleSheet| $b;
            (constr)(&mut style);
            style
        };);
	};
}
