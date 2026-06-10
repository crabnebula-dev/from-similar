use from_similar::FromSimilar;

/*
	A collection of tests that *compile* and ensure things work.
	As opposed to tests that expect compile errors, which need different test setups.

	Testing tips.

	Example owned types you can convert both ways in Rust std:
	- `(T, T, T)` from and into `[T; 3]` for various lengths.

	Example owned types that can convert only one way in Rust std:
	- `[T; N]` into `Vec<T>`
	- `u16` into `u32`

	`String` into `Arc<str>` also works one-way because has to deref to `&str` first before into `String` works.
	But since Deref implications make it rather obtuse, it's not recommended for testing here.
*/

#[test]
fn unit_struct_bidirectional() {
	struct UnitA;

	#[derive(FromSimilar)]
	#[from(UnitA, bidirectional = true)]
	struct UnitB;

	let a = UnitA;
	let b: UnitB = a.into();
	let _: UnitA = b.into();

	// Note: asserting the types here would be redundant.
	// There's also no scenario when bidirectional would *not* work.
}

// Define this struct as something we can make a qualified path to like `self::T`.
struct UnitInModule;

#[test]
fn qualified_paths_bidirectional() {
	// Something unexpected in an early version was the `syn::Ident` worked, but `syn::Path` did not.
	// Aka `#[from(T)]` was fine but `#[from(self::T)]` gave a compile error.
	// That shouldn't be the case, so we test it here.

	#[derive(FromSimilar)]
	#[from(self::UnitInModule, bidirectional = true)]
	struct UnitB;

	let a = self::UnitInModule;
	let b: UnitB = a.into();
	let _: UnitInModule = b.into();

	// Note: asserting the types here would be redundant.
	// There's also no scenario when bidirectional would *not* work.
}

#[test]
fn tuple_struct() {
	struct TupA(u16);

	#[derive(FromSimilar)]
	#[from(TupA, bidirectional = false)]
	struct TupB(u16);

	let a = TupA(42);

	let b: TupB = a.into();
	assert_eq!(b.0, 42);
}

#[test]
fn tuple_struct_bidirectional() {
	struct TupA(u16);

	#[derive(FromSimilar)]
	#[from(TupA, bidirectional = true)]
	struct TupB(u16);

	let a = TupA(42);

	let b: TupB = a.into();
	assert_eq!(b.0, 42);

	let c: TupA = b.into();
	assert_eq!(c.0, 42);
}

#[test]
fn tuple_struct_use_into() {
	struct TupA(u16, u16);

	#[derive(FromSimilar, Debug, PartialEq)]
	#[from(TupA, bidirectional = false)]
	struct TupB(u16, #[use_into] u32);

	let a = TupA(21, 42);

	let b: TupB = a.into();
	assert_eq!(b, TupB(21_u16, 42_u32));
}

#[test]
fn tuple_struct_bidirectional_use_into() {
	struct TupA([u16; 2]);

	#[derive(FromSimilar)]
	#[from(TupA, bidirectional = true)]
	struct TupB(#[use_into] (u16, u16));

	let a = TupA([21, 42]);

	let b: TupB = a.into();
	assert_eq!(b.0, (21, 42));

	let c: TupA = b.into();
	assert_eq!(c.0, [21, 42]);
}

#[test]
fn tuple_struct_bidirectional_use_into_option() {
	struct TupA(Option<[u16; 2]>);

	#[derive(FromSimilar)]
	#[from(TupA, bidirectional = true)]
	struct TupB(#[use_into_option] Option<(u16, u16)>);

	let a = TupA(Some([21, 42]));

	let b: TupB = a.into();
	assert_eq!(b.0, Some((21, 42)));

	let c: TupA = b.into();
	assert_eq!(c.0, Some([21, 42]));
}

#[test]
fn tuple_struct_bidirectional_use_into_collection() {
	struct TupA(Vec<[u16; 2]>);

	#[derive(FromSimilar)]
	#[from(TupA, bidirectional = true)]
	struct TupB(#[use_into_collection] Vec<(u16, u16)>);

	let a = TupA(vec![[21, 42]]);

	let b: TupB = a.into();
	assert_eq!(b.0, vec![(21, 42)]);

	let c: TupA = b.into();
	assert_eq!(c.0, vec![[21, 42]]);
}

#[test]
fn named_struct() {
	struct NamedA {
		x: u16,
	}

	#[derive(FromSimilar)]
	#[from(NamedA, bidirectional = false)]
	struct NamedB {
		x: u16,
	}

	let a = NamedA { x: 42 };

	let b: NamedB = a.into();
	assert_eq!(b.x, 42);
}

#[test]
fn named_struct_bidirectional() {
	struct NamedA {
		x: u16,
	}

	#[derive(FromSimilar)]
	#[from(NamedA, bidirectional = true)]
	struct NamedB {
		x: u16,
	}

	let a = NamedA { x: 42 };

	let b: NamedB = a.into();
	assert_eq!(b.x, 42);

	let c: NamedA = b.into();
	assert_eq!(c.x, 42);
}

#[test]
fn named_struct_use_into() {
	struct NamedA {
		x: u16,
		y: u16,
	}

	#[derive(FromSimilar)]
	#[from(NamedA, bidirectional = false)]
	struct NamedB {
		#[use_into]
		x: u32,
		y: u16,
	}

	let a = NamedA { x: 42, y: 21 };

	let b: NamedB = a.into();
	assert_eq!(b.x, 42_u32);
	assert_eq!(b.y, 21_u16);
}

#[test]
fn named_struct_bidirectional_use_into() {
	struct NamedA {
		pair: (u16, u16),
	}

	#[derive(FromSimilar)]
	#[from(NamedA, bidirectional = true)]
	struct NamedB {
		#[use_into]
		pair: [u16; 2],
	}

	let a = NamedA { pair: (21, 42) };

	let b: NamedB = a.into();
	assert_eq!(b.pair, [21, 42]);

	let c: NamedA = b.into();
	assert_eq!(c.pair, (21, 42));
}

#[test]
fn named_struct_bidirectional_use_into_option() {
	struct NamedA {
		pair: Option<(u16, u16)>,
	}

	#[derive(FromSimilar)]
	#[from(NamedA, bidirectional = true)]
	struct NamedB {
		#[use_into_option]
		pair: Option<[u16; 2]>,
	}

	let a = NamedA {
		pair: Some((21, 42)),
	};

	let b: NamedB = a.into();
	assert_eq!(b.pair, Some([21, 42]));

	let c: NamedA = b.into();
	assert_eq!(c.pair, Some((21, 42)));
}

#[test]
fn named_struct_bidirectional_use_into_collection() {
	struct NamedA {
		pair: Vec<(u16, u16)>,
	}

	#[derive(FromSimilar)]
	#[from(NamedA, bidirectional = true)]
	struct NamedB {
		#[use_into_collection]
		pair: Vec<[u16; 2]>,
	}

	let a = NamedA {
		pair: vec![(21, 42)],
	};

	let b: NamedB = a.into();
	assert_eq!(b.pair, vec![[21, 42]]);

	let c: NamedA = b.into();
	assert_eq!(c.pair, vec![(21, 42)]);
}

#[test]
fn input_inferred_lifetime_support() {
	struct NamedA<'a> {
		text: &'a str,
	}

	#[derive(FromSimilar)]
	#[from(input = NamedA<'_>)]
	struct NamedB {
		#[use_into]
		text: String,
	}

	let a = NamedA {
		text: "sample text",
	};

	let b: NamedB = a.into();
	assert_eq!(b.text, "sample text");
}

#[test]
fn input_generic_support() {
	struct NamedA<S> {
		text: S,
	}

	#[derive(FromSimilar)]
	#[from(input = NamedA<String>)]
	struct NamedB {
		#[use_into]
		text: String,
	}

	let a = NamedA {
		text: "sample text".to_string(),
	};

	let b: NamedB = a.into();
	assert_eq!(b.text, "sample text");
}
