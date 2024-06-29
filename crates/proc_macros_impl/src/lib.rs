use quote::{format_ident, quote};
use syn::{ItemStruct, Type, TypePath};

pub fn derive_setters(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
	let mut output = proc_macro2::TokenStream::new();

	let strct: ItemStruct = syn::parse2(input).expect("parse");
	let strct_type = strct.ident;
	for field in strct.fields.iter() {
		let field_type = &field.ty;
		let field_name = field.ident.as_ref().expect("named field");

		let getter_name = format_ident!("{field_name}");
		let setter_name = format_ident!("set_{field_name}");

		// Setter/Getters for Mut<T>
		if let Some(inner_type) = inner_type(field_type, "Mut") {
			output.extend(quote! {
				impl #strct_type{
					pub fn #getter_name(&self) -> #inner_type{
						self.#field_name.get()
					}

					pub fn #setter_name(&self, v: #inner_type){
						self.#field_name.set(v)
					}
				}
			});
		// Setters/Getters for <primitive>.
		} else if is_simple_type(field_type) {
			output.extend(quote! {
				impl #strct_type{
					pub fn #getter_name(&self) -> #field_type{
						self.#field_name
					}

					pub fn #setter_name(&mut self, v: #field_type){
						self.#field_name = v;
					}
				}
			});
		// Setters/Getters for <NonCopy>
		} else {
			output.extend(quote! {
				impl #strct_type{
					pub fn #getter_name(&self) -> &#field_type{
						&self.#field_name
					}

					pub fn #setter_name(&mut self, v: #field_type){
						self.#field_name = v;
					}
				}
			});
		}
	}
	output
}

/// If `ty` is of the form `Outer<Inner>` ("Outer" passed as string), return `Inner`.
/// E.g.:
///        inner_type(Vec<u32>, "Vec") => Some(u32)
///        inner_type(Vec<u32>, "Box") => None
fn inner_type<'t>(ty: &'t Type, outer: &str) -> Option<&'t Type> {
	if let Type::Path(TypePath { qself: None, path }) = ty {
		if path.segments.len() == 1 {
			let segment = &path.segments[0];
			if segment.ident == outer {
				if let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments {
					if arguments.args.len() == 1 {
						let argument = &arguments.args[0];
						if let syn::GenericArgument::Type(t) = argument {
							return Some(t);
						}
					}
				}
			}
		}
	}
	None
}

fn is_simple_type(ty: &Type) -> bool {
	if let Type::Path(TypePath { qself: None, path }) = ty {
		if path.segments.len() == 1 {
			let segment = &path.segments[0];
			if segment.ident.to_string().chars().all(|c| c == c.to_ascii_lowercase()) {
				return true;
			}
		}
	}
	false
}

/*
#[proc_macro_derive(EguiInspect, attributes(inspect))]
pub fn derive_egui_inspect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = input.ident;

	let generics = add_trait_bounds(input.generics);
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let inspect = inspect_struct(&input.data, &name, false);

	let inspect_mut = inspect_struct(&input.data, &name, true);

	let expanded = quote! {
		impl #impl_generics egui_inspect::EguiInspect for #name #ty_generics #where_clause {
			fn inspect(&self, label: &str, ui: &mut egui::Ui) {
				#inspect
			}
			fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
				#inspect_mut
			}
		}
	};

	proc_macro::TokenStream::from(expanded)
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
	for param in &mut generics.params {
		if let GenericParam::Type(ref mut type_param) = *param {
			type_param
				.bounds
				.push(parse_quote!(egui_inspect::EguiInspect));
		}
	}
	generics
}

fn inspect_struct(data: &Data, _struct_name: &Ident, mutable: bool) -> TokenStream {
	match *data {
		Data::Struct(ref data) => match data.fields {
			Fields::Named(ref fields) => handle_named_fields(fields, mutable),
			Fields::Unnamed(ref fields) => {
				let mut recurse = Vec::new();
				for (i,_) in fields.unnamed.iter().enumerate() {
					let tuple_index = Index::from(i);
					let name = format!("{i}");
					let ref_str = if mutable { quote!(&mut) } else { quote!(&) };
					recurse.push(quote! { egui_inspect::EguiInspect::inspect(#ref_str self.#tuple_index, #name, ui);});
				};

				let result = quote! {
					ui.strong(label);
					#(#recurse)*
				};
				result
			}
			_ => unimplemented!("Unit cannot be inspected !")
		},
		Data::Enum(_) | Data::Union(_) => unimplemented!("Enums and Unions are not yet supported"),
	}
}

fn handle_named_fields(fields: &FieldsNamed, mutable: bool) -> TokenStream {
	let recurse = fields.named.iter().map(|f| {
		let attr = AttributeArgs::from_field(f).expect("Could not get attributes from field");

		if attr.hide {
			return quote!();
		}

		let mutable = mutable && !attr.no_edit;

		if let Some(ts) = handle_custom_func(&f, mutable, &attr) {
			return ts;
		}

		if let Some(ts) = internal_paths::try_handle_internal_path(&f, mutable, &attr) {
			return ts;
		}

		return utils::get_default_function_call(&f, mutable, &attr);
	});
	quote! {
		ui.strong(label);
		#(#recurse)*
	}
}

fn handle_custom_func(field: &Field, mutable: bool, attrs: &AttributeArgs) -> Option<TokenStream> {
	let name = &field.ident;

	let name_str = match &attrs.name {
		Some(n) => n.clone(),
		None => name.clone().unwrap().to_string(),
	};

	if mutable && !attrs.no_edit && attrs.custom_func_mut.is_some() {
		let custom_func_mut = attrs.custom_func_mut.as_ref().unwrap();
		let ident = syn::Path::from_string(custom_func_mut)
			.expect(format!("Could not find function: {}", custom_func_mut).as_str());
		return Some(quote_spanned! { field.span() => {
				#ident(&mut self.#name, &#name_str, ui);
			}
		});
	}

	if (!mutable || (mutable && attrs.no_edit)) && attrs.custom_func.is_some() {
		let custom_func = attrs.custom_func.as_ref().unwrap();
		let ident = syn::Path::from_string(custom_func)
			.expect(format!("Could not find function: {}", custom_func).as_str());
		return Some(quote_spanned! { field.span() => {
				#ident(&self.#name, &#name_str, ui);
			}
		});
	}

	return None;
}

pub fn derive_jailed(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
	let mut output = proc_macro2::TokenStream::new();

	let strct: ItemStruct = syn::parse2(input).expect("parse");
	let strct_type = strct.ident;
	for field in strct.fields.iter() {
		let field_name = field.ident.as_ref().expect("named field");
		let field_type = &field.ty;
		let getter_name = format_ident!("get_{field_name}");
		let setter_name = format_ident!("set_{field_name}");

		output.extend(quote! {
			impl Jail<#strct_type>{
				pub fn #getter_name(&self) -> #field_type{
					self.borrow_mut().#field_name.clone()
				}

				pub fn #setter_name(&self, v: #field_type){
					self.borrow_mut().#field_name = v;
				}
			}
		});
	}
	output
}
*/
