use from_similar::FromSimilar;

struct UnitA;

#[derive(FromSimilar)]
// The input type is being defined twice.
#[from(UnitA, input = UnitA)]
struct UnitB;

fn main() {}
