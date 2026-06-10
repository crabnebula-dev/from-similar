use from_similar::FromSimilar;

struct UnitA;

#[derive(FromSimilar)]
#[from(UnitA, bidirectional = false)]
struct UnitB;

fn main() {
	let a = UnitA;
	let b: UnitB = a.into();
	// Attempting to convert both ways when `bidirectional = false`.
	let _: UnitA = b.into();
}
