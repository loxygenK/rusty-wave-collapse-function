#[macro_export]
macro_rules! define_css {
    ($name: ident, $css: expr) => {
        pub fn $name() -> String {
            use stylist::style;

            $css.unwrap().get_class_name().to_string()
        }
    };
}

#[macro_export]
macro_rules! css_fn {
    ($name: ident ( $($arg_ident: ident: $arg_type: ty), + ) $fn_body: expr ) => {
        pub fn $name($($arg_ident: $arg_type), +) -> String {
            use stylist::Style;

            fn generate_css($($arg_ident: $arg_type), +) -> String {
                $fn_body
            }

            Style::new(generate_css($($arg_ident), +))
                .unwrap()
                .get_class_name()
                .to_string()
        }
    }
}
