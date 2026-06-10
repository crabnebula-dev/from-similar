use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DeriveInput, Field, FieldsNamed, FieldsUnnamed, Index, LitBool};

// Everything has the "UseInto" prefix because those are the actual attribute names.
#[allow(clippy::enum_variant_names)]
enum FieldAttr {
	UseInto,
	UseIntoCollection,
	UseIntoOption,
}

impl FieldAttr {
	fn find_first(field: &Field) -> Option<Self> {
		field.attrs.iter().find_map(|a| match a.path() {
			p if p.is_ident("use_into") => Some(FieldAttr::UseInto),
			p if p.is_ident("use_into_collection") => Some(FieldAttr::UseIntoCollection),
			p if p.is_ident("use_into_option") => Some(FieldAttr::UseIntoOption),
			_ => None,
		})
	}
}

fn maybe_suffix_for(field: &Field) -> TokenStream {
	match FieldAttr::find_first(field) {
		Some(FieldAttr::UseInto) => quote!(.into()),
		Some(FieldAttr::UseIntoCollection) => quote!(.into_iter().map(Into::into).collect()),
		Some(FieldAttr::UseIntoOption) => quote!(.map(Into::into)),
		None => quote!(),
	}
}

pub fn expand_from_similar(
	DeriveInput {
		attrs,
		data,
		ident: self_ident,
		generics,
		..
	}: DeriveInput,
) -> syn::Result<TokenStream> {
	// Guard against other types like enums that we don't support.
	let syn::Data::Struct(data) = &data else {
		return Err(syn::Error::new(
			self_ident.span(),
			"#[derive(FromSimilar)] only supports converting between structs",
		));
	};

	if !generics.params.is_empty() {
		return Err(syn::Error::new_spanned(
			generics,
			"#[derive(FromSimilar)] currently doesn't support generics on the deriving type, but limited support on the input type exists `#[from(input = SomeType<String>)]`",
		));
	}

	// Finds the required `#[from]`.
	let from_attr = attrs
		.iter()
		.find(|a| a.path().is_ident("from"))
		.ok_or(syn::Error::new(
			self_ident.span(),
			"#[derive(FromSimilar)] requires a #[from()] attribute",
		))?;

	// Pull the arguments out of the `#[from]`.

	// `input_ident` is required and will cause a compile error later if not Some().
	// Can be either shorthand #[from(InputType)] or named #[from(input = InputType)].
	let mut input_ident = None::<syn::Path>;
	let mut bidirectional = None::<bool>;

	from_attr
		.parse_nested_meta(|m| {
			// Attempt to parse named argument `#[from(.., path = value)]`.
			match m.value() {
				// Failing to parse the `=` we'll interpret as #[from(InputType)].
				Err(_) => {
					if input_ident.is_some() {
						return Err(m.error("Input type is already defined"));
					}
					input_ident = Some(m.path);
					Ok(())
				}
				// An `=` was parsed, interpret based on argument name.
				// #[from(input = InputType)]
				Ok(val) if m.path.is_ident("input") => {
					if input_ident.is_some() {
						return Err(m.error("Input type is already defined"));
					}
					input_ident = Some(val.parse()?);
					Ok(())
				}
				// #[from(..., bidirectional = true)]
				Ok(val) if m.path.is_ident("bidirectional") => {
					if bidirectional.is_some() {
						return Err(m.error("Bidirectional parameter is already defined"));
					}
					let b: LitBool = val.parse()?;
					bidirectional = Some(b.value);
					Ok(())
				}
				_ => Err(m.error("Unrecognized parameter")),
			}
		})
		.map_err(|err| {
			// Add hint for strange parsing edge case where `#[from(MyStruct<'_>)]`
			// fails but `#[from(input = MyStruct<'_>)]` works.
			if err.span().source_text() == Some("<".to_string()) && err.to_string() == "expected `,`" {
				let example_ident = match &input_ident {
					Some(input_ident) => input_ident.clone().to_token_stream().to_string(),
					None => "MyType".to_string(),
				};
				return syn::Error::new_spanned(
					from_attr,
					format!("Unsupported generic arguments in the shorthand form. Try using `#[from(input = {example_ident}<'_>)]` instead."),
				);
			}
			err
		})?;

	let input_ident = input_ident
		.expect("#[from()] missing required input type argument, either as `input = T` or `T`");
	let bidirectional = bidirectional.unwrap_or(false);

	// How to create a `Self` in the from function.
	// This is the same in either direction, but changes for the kind of struct.
	let literal_self_block = match &data.fields {
		// Unit / zero-size structs are simple.
		// Example: `Self`
		syn::Fields::Unit => quote! {
			Self
		},

		// Tuple structs use Index for it's fields.
		// Example: `Self ( value.0 )`
		syn::Fields::Unnamed(FieldsUnnamed {
			unnamed: fields, ..
		}) => {
			let (indexes, maybe_suffixes): (Vec<_>, Vec<_>) = fields
				.iter()
				.enumerate()
				.map(|(i, f)| (Index::from(i), maybe_suffix_for(f)))
				.unzip();

			quote! {
				Self (
					#(value.#indexes #maybe_suffixes,)*
				)
			}
		}

		// "Normal" named structs use the Ident to access fields.
		// Example: `Self { x: value.x }`
		syn::Fields::Named(FieldsNamed { named: fields, .. }) => {
			let (fields, maybe_suffixes): (Vec<_>, Vec<_>) = fields
				.iter()
				.map(|f| {
					let ident = f.ident.as_ref().unwrap();
					(ident, maybe_suffix_for(f))
				})
				.unzip();

			quote! {
				Self {
					#(#fields: value.#fields #maybe_suffixes,)*
				}
			}
		}
	};

	let from_input = quote! {
		#[automatically_derived]
		impl From<#input_ident> for #self_ident {
			fn from(value: #input_ident) -> Self {
				#literal_self_block
			}
		}
	};

	let maybe_from_self = if bidirectional {
		quote! {
			#[automatically_derived]
			impl From<#self_ident> for #input_ident {
				fn from(value: #self_ident) -> Self {
					#literal_self_block
				}
			}
		}
	} else {
		quote!()
	};

	Ok(quote! {
		#from_input
		#maybe_from_self
	})
}
