use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr};
use quote::quote;

#[proc_macro]
pub fn my_power(_item: TokenStream) -> TokenStream {
    // ****1st step***
    //let item = _item.clone();
    //dbg!(&item);
    //item

    // ***2nd step***
    //let item = _item.clone();
    //let ast = parse_macro_input!(_item as Expr);
    //dbg!(&ast);
    //item

    // ***3rd step***
    let ast = parse_macro_input!(_item as Expr);
    let res = quote! {
        (#ast) * (#ast)
    };
    res.into()
}