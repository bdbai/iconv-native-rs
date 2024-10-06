#[cfg(target_arch = "wasm32")]
macro_rules! with_harness {
    ($($tt:item)*) => {
        $(
            #[wasm_bindgen_test::wasm_bindgen_test]
            $tt
        )*
    };
}

#[cfg(not(target_arch = "wasm32"))]
macro_rules! with_harness {
    ($($tt:item)*) => {
        $(
            #[test]
            $tt
        )*
    };
}
