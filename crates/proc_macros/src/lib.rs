#[proc_macro_derive(Setters, attributes(inspect))]
pub fn derive_setters(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	proc_macros_impl::derive_setters(input.into()).into()
}
