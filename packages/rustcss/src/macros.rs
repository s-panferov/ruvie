// pub use enclose::*;

#[macro_export]
macro_rules! style {
	($name:ident, item) => {
        struct $name {

        };

        impl $name {
            fn name() -> &'static str {
                std::stringify!($name)
            }
        }

		lazy_static::lazy_static!(static ref $name: StyleSheet = {
            let mut style = rustcss::StyleSheet::new();
            let constr = |$ctx: &mut StyleSheet| { $($b)* };
            (constr)(&mut style);
            style
        };);
    };
}

#[macro_export]
macro_rules! style2 {
	($name:ident, |$ctx:ident| $b:block) => {
		lazy_static::lazy_static!(static ref $name: StyleSheet = {
            let mut style = rustcss::StyleSheet::new();
            style.with_name(std::stringify!($name));
            let constr = move |$ctx: &mut StyleSheet| $b;
            (constr)(&mut style);
            style
        };);
	};
}
