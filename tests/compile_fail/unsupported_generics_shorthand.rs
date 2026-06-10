use from_similar::FromSimilar;

struct NamedA<'a> {
	text: &'a str,
}

#[derive(FromSimilar)]
// This *would* work in the `input = T<'_>` format, but not here.
#[from(NamedA<'_>)]
struct NamedB {
	#[use_into]
	text: String,
}

#[derive(FromSimilar)]
// This is fine though.
#[from(input = NamedA<'_>, bidirectional = false)]
struct NamedC {
	#[use_into]
	text: String,
}

fn main() {}
