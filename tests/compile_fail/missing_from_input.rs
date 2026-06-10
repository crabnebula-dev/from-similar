use from_similar::FromSimilar;

struct UnitA;

#[derive(FromSimilar)]
// The `#[from]` attribute is missing the required input argument.
#[from()]
struct UnitB;

fn main() {}
