# from-similar ![License: Apache-2.0 OR MIT](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue) [![from-similar on crates.io](https://img.shields.io/crates/v/from-similar)](https://crates.io/crates/from-similar) [![from-similar on docs.rs](https://docs.rs/from-similar/badge.svg)](https://docs.rs/from-similar) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/crabnebula-dev/from-similar) [![Rust Version: 1.71.1](https://img.shields.io/badge/rustc-1.71.1-orange.svg)](https://github.com/rust-lang/rust/releases/tag/1.71.1)

`FromSimilar` automatically implements [`From`][__link0] between two structs that are “similar”.

Specifically for structs where the *fields have the same names*.
Or tuple structs with similar positional arguments.

This macro is mainly useful to generate predicable `From` implementations for:

* Structs that are *mostly* identical, except for a few attributes like `#[serde]` exceptions for serializing BSON.
* Structs with a subset of fields from the more complete one.

#### Struct attributes

`#[from(InputType)]` a **required attribute** to specify the input type.<br>
Will generate `impl From<InputType> for T`.

`#[from(.., bidirectional = true)]` optional attribute to implement both directions.<br>
Will generate `impl From<InputType> for T` and `impl From<T> for InputType`.

#### Field attributes

`#[use_into]` is an optional *field attribute* to use `.into()` when converting this field.

`#[use_into_option]` for `Option<T>` types that need a `.map(Into::into)` when converting this field.

`#[use_into_collection]` for `impl IntoIterator<T>` types that should map each item and collect.

### Example with database models

A bidirectional FromSimilar that can be used for MongoDB.

```rust
use from_similar::FromSimilar;

#[derive(Default)]
struct NormalModel {
    id: String,
    date: chrono::DateTime<chrono::Utc>,
    date_option: Option<chrono::DateTime<chrono::Utc>>,
    date_list: Vec<chrono::DateTime<chrono::Utc>>,
}

#[derive(FromSimilar, serde::Serialize, serde::Deserialize)]
#[from(NormalModel, bidirectional = true)]
struct DatabaseModel {
    #[serde(rename = "_id")]
    id: String,

    #[use_into]
    date: bson::DateTime,

    #[use_into_option]
    date_option: Option<bson::DateTime>,

    #[use_into_collection]
    date_list: Vec<bson::DateTime>,
}

let normal = NormalModel::default();
let db: DatabaseModel = normal.into();
let _: NormalModel = db.into();
```

#### Example with views

Note: `#[from(.., bidirectional = true)]` would break here, because it’s a lossy conversion.

```rust
use from_similar::FromSimilar;

#[derive(Default)]
struct FullModel {
    id: String,
    pretty_name: String,
    secret: String,
}

#[derive(FromSimilar)]
#[from(FullModel)]
struct PublicView {
    id: String,
    pretty_name: String,
    // ... omits `secret` field
}

let full = FullModel::default();
let _: PublicView = full.into();
```

#### Example with tuple struct

```rust
use from_similar::FromSimilar;

#[derive(Default)]
struct Data(pub String, pub usize);

#[derive(FromSimilar)]
#[from(Data)]
struct SealedData(#[use_into] std::sync::Arc<str>, usize);

let mut data = Data::default();
data.0 += "Pushing ";
data.0 += "some text";
data.1 = 42;
let data: SealedData = data.into();
```


 [__link0]: https://doc.rust-lang.org/stable/std/convert/trait.From.html
