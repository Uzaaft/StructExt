use proc_macro::TokenStream;

mod iterator;

#[proc_macro_derive(IterateFields)]
pub fn iterator_derive(input: TokenStream) -> TokenStream {
    iterator::iterate_fields_derive(input)
}
