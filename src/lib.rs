#![deny(unsafe_code)]
#![deny(missing_docs)]
#![allow(clippy::tabs_in_doc_comments)]

//! `FromSimilar` automatically implements [`From`] between two structs that are "similar".
//!
//! Specifically for structs where the *fields have the same names*.
//! Or tuple structs with similar positional arguments.
//!
//! This macro is mainly useful to generate predicable `From` implementations for:
//! - Structs that are *mostly* identical, except for a few attributes like `#[serde]` exceptions for serializing BSON.
//! - Structs with a subset of fields from the more complete one.
//!
//! ### Struct attributes
//!
//! `#[from(InputType)]` a **required attribute** to specify the input type.<br>
//! Will generate `impl From<InputType> for T`.
//!
//! `#[from(.., bidirectional = true)]` optional attribute to implement both directions.<br>
//! Will generate `impl From<InputType> for T` and `impl From<T> for InputType`.
//!
//! ### Field attributes
//!
//! `#[use_into]` is an optional *field attribute* to note that `.into()` should be called when converting this field.
//!
//! ## Example with database models
//!
//! A bidirectional FromSimilar that can be used for MongoDB.
//!
//! ```rust
//! use from_similar::FromSimilar;
//!
//! #[derive(Default)]
//! struct NormalModel {
//!     id: String,
//!     date: chrono::DateTime<chrono::Utc>,
//! }
//!
//! #[derive(FromSimilar, serde::Serialize, serde::Deserialize)]
//! #[from(NormalModel, bidirectional = true)]
//! struct DatabaseModel {
//!     #[serde(rename = "_id")]
//!     id: String,
//!
//!     #[use_into]
//!     date: bson::DateTime,
//! }
//!
//! let normal = NormalModel::default();
//! let db: DatabaseModel = normal.into();
//! let _: NormalModel = db.into();
//! ```
//!
//! ### Example with views
//!
//! Note: `#[from(.., bidirectional = true)]` would break here, because it's a lossy conversion.
//!
//! ```rust
//! use from_similar::FromSimilar;
//!
//! #[derive(Default)]
//! struct FullModel {
//!     id: String,
//!     pretty_name: String,
//!     secret: String,
//! }
//!
//! #[derive(FromSimilar)]
//! #[from(FullModel)]
//! struct PublicView {
//!     id: String,
//!     pretty_name: String,
//!     // ... omits `secret` field
//! }
//!
//! let full = FullModel::default();
//! let _: PublicView = full.into();
//! ```
//!
//! ### Example with tuple struct
//!
//! ```rust
//! use from_similar::FromSimilar;
//!
//! #[derive(Default)]
//! struct Data(pub String, pub usize);
//!
//! #[derive(FromSimilar)]
//! #[from(Data)]
//! struct SealedData(#[use_into] std::sync::Arc<str>, usize);
//!
//! let mut data = Data::default();
//! data.0 += "Pushing ";
//! data.0 += "some text";
//! data.1 = 42;
//! let data: SealedData = data.into();
//! ```

extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod from_similar;

/// [`FromSimilar`] derive macro.
///
/// Typical usage means adding the Derive macro, setting the source with
/// `#[from(SourceType)]` and using field macros as needed.
///
/// - `#[use_into]` for types that need direct `into` calls.
/// - `#[use_into_option]` for `Option<T>` types that need a map-into.
/// - `#[use_into_collection]` for `impl IntoIterator<T>` types that should map each item and collect.
#[proc_macro_derive(
	FromSimilar,
	attributes(from, use_into, use_into_option, use_into_collection)
)]
pub fn from_similar(input: TokenStream) -> TokenStream {
	let ty = parse_macro_input!(input as DeriveInput);
	from_similar::expand_from_similar(ty)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}
