extern crate proc_macro;

use self::proc_macro::TokenStream;

#[proc_macro_derive(A)] // declare derive:A
pub fn derive(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    assert!(input.contains("struct A"));

    r#"
        impl A {
            fn a(&self) -> String {
                format!("hello from impl A")
            }
        }
    "#
    .parse()
    .unwrap()
}
