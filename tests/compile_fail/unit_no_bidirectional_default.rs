use from_similar::FromSimilar;

struct UnitA;

#[derive(FromSimilar)]
#[from(UnitA)]
struct UnitB;

fn main() {
	let a = UnitA;
	let b: UnitB = a.into();
	// Attempting to convert both ways when `bidirectional = false` is the default behavior.
	let _: UnitA = b.into();
}
