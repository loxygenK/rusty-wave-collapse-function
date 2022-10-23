#[macro_export]
macro_rules! enums {
    ($source:ty : { $( $name:ident ), * }) => {
        vec![ $( <$source>::$name, ) * ]
    };
}
