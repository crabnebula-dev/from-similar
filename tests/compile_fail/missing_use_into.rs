use from_similar::FromSimilar;

struct NamedA {
	x: u16,
}

#[derive(FromSimilar)]
#[from(NamedA)]
struct NamedB {
	// Different field type without explicit #[use_into]
	x: u32,
}

fn main() {}
