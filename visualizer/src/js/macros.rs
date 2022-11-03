macro_rules! bind_js {
    ($namespace: ident . $name: ident ( $( $arg_ident: ident : $arg_type: ty ), + )) => {
        pub mod $namespace {
            use wasm_bindgen::prelude::*;

            pub fn $name($( $arg_ident: $arg_type ), +) {
                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(js_namespace = $namespace, js_name = $name)]
                    pub fn binded($( $arg_ident: $arg_type ), +);
                }

                binded($( $arg_ident ), +);
            }
        }
    };
}
