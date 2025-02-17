use montgomery_reduction::acar::cios_opt;
use montgomery_reduction::{cios, fios, sos, subtraction_step, NP0, P};

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

    let (a, b) = ([1, 0, 0, 0], [1, 0, 0, 0]);
    let d = cios_opt(a, b, P, NP0);
    let c = cios_opt(
        [
            0x1BB8E645AE216DA7,
            0x53FE3AB1E35C59E3,
            0x8C49833D53BB8085,
            0x0216D0B17F4E44A5,
        ],
        d[0..4].try_into().unwrap(),
        P,
        NP0,
    );
    println!(
        "cios_opt: \t{:?}",
        subtraction_step(c[..4].try_into().unwrap(), P)
    );
    println!("");
    let (a, b) = ([0, 0, 0, 6973996533605941332], [1, 0, 0, 0]);
    let c = cios_opt(
        [
            0x1BB8E645AE216DA7,
            0x53FE3AB1E35C59E3,
            0x8C49833D53BB8085,
            0x0216D0B17F4E44A5,
        ],
        a,
        P,
        NP0,
    );
    let d = cios_opt(c[..4].try_into().unwrap(), b, P, NP0);
    println!("d: \t{:?}", d);
    println!("a: \t{:?}", a);
    println!("a: \t{:?}", subtraction_step(subtraction_step(a, P), P));
    println!(
        "cios_opt: \t{:?}",
        subtraction_step(d[..4].try_into().unwrap(), P)
    );
    println!("P: {P:?}");
}
