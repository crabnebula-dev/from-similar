use from_similar::FromSimilar;

struct NamedA {
	text: String,
}

#[derive(FromSimilar)]
#[from(NamedA)]
struct NamedB<S> {
	text: S,
}

fn main() {
	let a = NamedA {
		text: "example".to_string(),
	};

	let _b: NamedB<String> = a.into();
}
