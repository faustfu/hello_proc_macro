extern crate proc_macro;

use self::proc_macro::TokenStream;

#[proc_macro_derive(A)] // declare derive(A)
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

#[proc_macro_attribute]
pub fn attr_with_args(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = args.to_string();
    let input = input.to_string();
    assert!(input.contains("fn foo()"));

    format!("fn foo() -> &'static str {{ {} }}", args)
        .parse()
        .unwrap()
}
