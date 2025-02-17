use montgomery_reduction::{
    convert_limb_sizes, emmart, subtraction_step, subtraction_step_u52, U256b52, P, U52_NP0, U52_P,
    U52_R2,
};

fn main() {
    let a = U256b52([0, 0, 0, 0, 60023223353699]);
    let b = U256b52([1, 0, 0, 0, 0]);
    let a_tilde = emmart::sos(a, U256b52(U52_R2), U256b52(U52_P), U52_NP0);
    a_tilde.iter().for_each(|li| print!("{li:X}\t"));
    println!("");
    let a_round = emmart::sos(
        U256b52(a_tilde[5..].try_into().unwrap()),
        b,
        U256b52(U52_P),
        U52_NP0,
    );
    a_round.iter().for_each(|li| print!("{li:X}\t"));
    println!("");
    let mut d = a.0;
    let mut prev = d;
    loop {
        d = subtraction_step_u52(d, U52_P);
        if d == prev {
            break;
        }
        prev = d;
    }
    print!("a: ");
    d.iter().for_each(|li| print!("{li:X}\t"));
    println!("");

    let mut d = a_round[5..].try_into().unwrap();
    let mut prev = d;
    loop {
        d = subtraction_step_u52(d, U52_P);
        if d == prev {
            break;
        }
        prev = d;
    }
    d.iter().for_each(|li| print!("{li:X}\t"));
    println!("");
    // Or first calculate inverse and then do the convert_limb_sizes
}
