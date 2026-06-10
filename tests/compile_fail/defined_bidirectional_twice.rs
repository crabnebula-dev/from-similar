use from_similar::FromSimilar;

struct UnitA;

#[derive(FromSimilar)]
// The bidirectional is being defined twice.
#[from(UnitA, bidirectional = true, bidirectional = false)]
struct UnitB;

fn main() {}
