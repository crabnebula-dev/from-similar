use from_similar::FromSimilar;

struct UnitA;

#[derive(FromSimilar)]
// There is no `#[from]`, while it's required.
struct UnitB;

fn main() {}
