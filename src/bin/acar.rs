use montgomery_reduction::{cios, cios_opt, fios, sos, NP0, P};

fn main() {
    let a = [0_u64; 4];
    let b = [0_u64; 4];

    let c = sos(a, b, P, NP0);
    println!("sos:      \t{c:?}");
    let c = fios(a, b, P, NP0);
    println!("fios:     \t{c:?}");
    let c = cios(a, b, P, NP0);
    println!("cios:     \t{c:?}");
    let c = cios_opt(a, b, P, NP0);
    println!("cios_opt: \t{c:?}\n");

    let a = [0, 0, 0, 1];
    let b = [0, 0, 0, 1];

    let c = sos(a, b, P, NP0);
    println!("sos:      \t{c:?}");
    let c = fios(a, b, P, NP0);
    println!("fios:     \t{c:?}");
    let c = cios(a, b, P, NP0);
    println!("cios:     \t{c:?}");
    let c = cios_opt(a, b, P, NP0);
    println!("cios_opt: \t{c:?}\n");

    let a = [1, 0, 0, 1];
    let b = [1, 0, 0, 1];

    let c = sos(a, b, P, NP0);
    println!("sos:      \t{c:?}");
    let c = fios(a, b, P, NP0);
    println!("fios:     \t{c:?}");
    let c = cios(a, b, P, NP0);
    println!("cios:     \t{c:?}");
    let c = cios_opt(a, b, P, NP0);
    println!("cios_opt: \t{c:?}\n");

    let (a, b) = (
        [0, 0, 0, 15041487139945544921],
        [64395813789477709, 0, 0, 18358496891515497855],
    );
    let c = sos(a, b, P, NP0);
    println!("sos:      \t{c:?}");
    let c = fios(a, b, P, NP0);
    println!("fios:     \t{c:?}");
    let c = cios(a, b, P, NP0);
    println!("cios:     \t{c:?}");
    let c = cios_opt(a, b, P, NP0);
    println!("cios_opt: \t{c:?}");
}
