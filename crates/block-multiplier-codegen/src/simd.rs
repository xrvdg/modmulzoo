use std::array;

use block_multiplier::{constants::*, make_initial};
use hla::*;
use montgomery_reduction::domb::heaviside;

use crate::scalar::{load_const, load_floating_simd};

pub fn setup_u256_to_u260_shl2_imd(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
) -> (Vec<FreshVariable>, FreshVariable) {
    let limbs = alloc.fresh_array();

    let mask = mov(alloc, asm, MASK52);
    let mask_simd = dup2d(alloc, asm, &mask);

    let var_limb = FreshVariable::new("limbs", &limbs);
    let res = u256_to_u260_shl2_simd(alloc, asm, &mask_simd, limbs);

    (vec![var_limb], FreshVariable::new("out", &res))
}

pub fn setup_u260_to_u256_simd(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
) -> (Vec<FreshVariable>, FreshVariable) {
    let limbs = alloc.fresh_array();

    let var_limb = FreshVariable::new("limbs", &limbs);
    let res = u260_to_u256_simd(alloc, asm, limbs);

    (vec![var_limb], FreshVariable::new("out", &res))
}

pub fn setup_vmultadd_noinit_simd(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
) -> (Vec<FreshVariable>, FreshVariable) {
    let t = alloc.fresh_array();
    let a = alloc.fresh_array();
    let b = alloc.fresh_array();

    let c1 = mov(alloc, asm, C1.to_bits());
    let c1 = dup2d(alloc, asm, &c1);

    // Alternative is c2 = c1 + 1; This requires a change to add to support immediate
    let c2 = load_const(alloc, asm, C2.to_bits());
    let c2 = dup2d(alloc, asm, &c2);
    let var_t = FreshVariable::new("t", &t);
    let var_a = FreshVariable::new("a", &a);
    let var_b = FreshVariable::new("b", &b);

    let res = vmultadd_noinit_simd(alloc, asm, &c1, &c2, t, a, b);

    (vec![var_t, var_a, var_b], FreshVariable::new("out", &res))
}

pub fn setup_single_step_simd(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
) -> (Vec<FreshVariable>, FreshVariable) {
    let a = alloc.fresh_array();
    let b = alloc.fresh_array(); // Assuming b starts after a

    let var_a = FreshVariable::new("av", &a);
    let var_b = FreshVariable::new("bv", &b);
    let res = single_step_simd(alloc, asm, a, b);

    (vec![var_a, var_b], FreshVariable::new("outv", &res))
}

pub fn setup_reduce_ct_simd(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
) -> (Vec<FreshVariable>, FreshVariable) {
    let red = alloc.fresh_array();

    let mask = mov(alloc, asm, MASK52);
    let mask52 = dup2d(alloc, asm, &mask);

    let var_red = FreshVariable::new("red", &red);

    let res = reduce_ct_simd(alloc, asm, red).map(|reg| and16(alloc, asm, &reg, &mask52));

    (vec![var_red], FreshVariable::new("out", &res))
}

//*******  SIMD **********/
fn load_tuple(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
    fst: Reg<u64>,
    snd: Reg<u64>,
) -> Reg<Simd<u64, 2>> {
    let fresh: Reg<Simd<u64, 2>> = alloc.fresh();
    asm.append_instruction(vec![
        ins_inst(fresh._d0(), &fst),
        ins_inst(fresh._d1(), &snd),
    ]);
    fresh
}

fn transpose_u256_to_simd(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
    limbs: [[Reg<u64>; 4]; 2],
) -> [Reg<Simd<u64, 2>>; 4] {
    let [[l00, l01, l02, l03], [l10, l11, l12, l13]] = limbs;
    [
        load_tuple(alloc, asm, l00, l10),
        load_tuple(alloc, asm, l01, l11),
        load_tuple(alloc, asm, l02, l12),
        load_tuple(alloc, asm, l03, l13),
    ]
}

fn u256_to_u260_shl2_simd(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
    mask52: &Reg<Simd<u64, 2>>,
    limbs: [Reg<Simd<u64, 2>>; 4],
) -> [Reg<Simd<u64, 2>>; 5] {
    let [l0, l1, l2, l3] = limbs;

    let shifted_l1 = shl2d(alloc, asm, &l1, 14);
    let shifted_l2 = shl2d(alloc, asm, &l2, 26);
    let shifted_l3 = shl2d(alloc, asm, &l3, 38);
    // The input and output interface share the same registers. By moving this operation to somewhere in the beginning
    // we can free up the hardware register tied to l3.
    let last = ushr2d(alloc, asm, &l3, 14);

    let shifted_ol0 = shl2d(alloc, asm, &l0, 2);
    let shifted_ol1 = usra2d(alloc, asm, shifted_l1, &l0, 50);
    let shifted_ol2 = usra2d(alloc, asm, shifted_l2, &l1, 38);
    let shifted_ol3 = usra2d(alloc, asm, shifted_l3, &l2, 26);

    [
        and16(alloc, asm, &shifted_ol0, &mask52),
        and16(alloc, asm, &shifted_ol1, &mask52),
        and16(alloc, asm, &shifted_ol2, &mask52),
        and16(alloc, asm, &shifted_ol3, &mask52),
        last,
    ]
}

fn load_const_simd(alloc: &mut FreshAllocator, asm: &mut Assembler, val: u64) -> Reg<Simd<u64, 2>> {
    let val = load_const(alloc, asm, val);
    let mask = dup2d(alloc, asm, &val);
    mask
}

// Embed the initials as instructions
// TODO with larger block size this loading can be kept outside and copied
// This is very specific to parallel_sub_simd_r256 is might be better inlined
fn make_initials(alloc: &mut FreshAllocator, asm: &mut Assembler) -> [Reg<Simd<u64, 2>>; 10] {
    let mut t: [Reg<Simd<u64, 2>>; 10] = array::from_fn(|_| alloc.fresh());

    for i in 0..5 {
        let lower_val = make_initial(i + 1 + 5 * heaviside(i as isize - 4), i);
        let lower_val = mov(alloc, asm, lower_val);

        t[i] = dup2d(alloc, asm, &lower_val);

        let j = 10 - 1 - i;

        let upper_val = make_initial(i + 5 * (1 - heaviside(j as isize - 9)), i + 1 + 5 * 1);
        let upper_val = mov(alloc, asm, upper_val);
        t[j] = dup2d(alloc, asm, &upper_val);
    }

    t
}

fn vmultadd_noinit_simd(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
    c1: &Reg<Simd<u64, 2>>,
    c2: &Reg<Simd<u64, 2>>,
    mut t: [Reg<Simd<u64, 2>>; 10],
    a: [Reg<Simd<u64, 2>>; 5],
    b: [Reg<Simd<u64, 2>>; 5],
) -> [Reg<Simd<u64, 2>>; 10] {
    let a = a.map(|ai| ucvtf2d(alloc, asm, &ai));
    let b = b.map(|bi| ucvtf2d(alloc, asm, &bi));
    for i in 0..a.len() {
        for j in 0..b.len() {
            let lc1 = mov16b(alloc, asm, c1);

            let hi = fmla2d(alloc, asm, lc1.into_(), &a[i], &b[j]);
            let tmp = fsub2d(alloc, asm, &c2.as_(), &hi);
            let lo = fmla2d(alloc, asm, tmp, &a[i], &b[j]);

            t[i + j + 1] = add2d(alloc, asm, &t[i + j + 1], &hi.into_());
            t[i + j] = add2d(alloc, asm, &t[i + j], &lo.into_());
        }
    }
    t
}

// Whole vector is in registers, but that might not be great. Better to have it on the stack and load it from there
pub fn smultadd_noinit_simd(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
    mut t: [Reg<Simd<u64, 2>>; 6],
    c1: &Reg<Simd<u64, 2>>,
    c2: &Reg<Simd<u64, 2>>,
    s: Reg<Simd<u64, 2>>,
    v: [u64; 5],
) -> [Reg<Simd<u64, 2>>; 6] {
    let s = ucvtf2d(alloc, asm, &s);

    // This ordering is the fastest that I've found. Any change or breaking up into parts seem
    // to inhibit bypass
    for i in 0..v.len() {
        // skip ucvtf by loading the constant directly as (simd) floating point
        // No measurable difference in loading the vector v completely outside or per element inside the load
        let vs = load_floating_simd(alloc, asm, v[i] as f64);
        let lc1 = mov16b(alloc, asm, &c1);

        let hi = fmla2d(alloc, asm, lc1.into_(), &s, &vs);
        let tmp = fsub2d(alloc, asm, c2.as_(), &hi);
        let lo = fmla2d(alloc, asm, tmp, &s, &vs);

        t[i + 1] = add2d(alloc, asm, &t[i + 1], hi.as_());
        t[i] = add2d(alloc, asm, &t[i], lo.as_());
    }
    t
}

/// Constants that are used across functions
/// Misses the transposing to make it easier on the registers for the next steps
fn single_step_simd(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
    a: [Reg<Simd<u64, 2>>; 4],
    b: [Reg<Simd<u64, 2>>; 4],
) -> [Reg<Simd<u64, 2>>; 4] {
    let mask = mov(alloc, asm, MASK52);
    let mask52 = dup2d(alloc, asm, &mask);

    let a = u256_to_u260_shl2_simd(alloc, asm, &mask52, a);
    let b = u256_to_u260_shl2_simd(alloc, asm, &mask52, b);
    let t = make_initials(alloc, asm);

    let c1 = mov(alloc, asm, C1.to_bits());
    let c1 = dup2d(alloc, asm, &c1);

    // Alternative is c2 = c1 + 1; This requires a change to add to support immediate
    let c2 = load_const(alloc, asm, C2.to_bits());
    let c2 = dup2d(alloc, asm, &c2);

    let [t0, t1, t2, t3, t4, t5, t6, t7, t8, t9] =
        vmultadd_noinit_simd(alloc, asm, &c1, &c2, t, a, b);

    let t1 = usra2d(alloc, asm, t1, &t0, 52);
    let t2 = usra2d(alloc, asm, t2, &t1, 52);
    let t3 = usra2d(alloc, asm, t3, &t2, 52);
    let t4 = usra2d(alloc, asm, t4, &t3, 52);

    let t4_10 = [t4, t5, t6, t7, t8, t9];

    let t0 = and16(alloc, asm, &t0, &mask52);
    let t1 = and16(alloc, asm, &t1, &mask52);
    let t2 = and16(alloc, asm, &t2, &mask52);
    let t3 = and16(alloc, asm, &t3, &mask52);

    // loading rho interleaved with multiplication to prevent to prevent allocation a lot of X-registers
    let r0 = smultadd_noinit_simd(alloc, asm, t4_10, &c1, &c2, t0, RHO_4);
    let r1 = smultadd_noinit_simd(alloc, asm, r0, &c1, &c2, t1, RHO_3);
    let r2 = smultadd_noinit_simd(alloc, asm, r1, &c1, &c2, t2, RHO_2);
    let s = smultadd_noinit_simd(alloc, asm, r2, &c1, &c2, t3, RHO_1);

    // Could be replaced with fmul, but the rust compiler generates something close to this
    let u52_np0 = load_const(alloc, asm, U52_NP0);
    let s00 = umov(alloc, asm, &s[0]._d0());
    let s01 = umov(alloc, asm, &s[0]._d1());
    let m0 = mul(alloc, asm, &s00, &u52_np0);
    let m1 = mul(alloc, asm, &s01, &u52_np0);

    let m0 = and(alloc, asm, &m0, &mask);
    let m1 = and(alloc, asm, &m1, &mask);
    let m = load_tuple(alloc, asm, m0, m1);

    let s = smultadd_noinit_simd(alloc, asm, s, &c1, &c2, m, U52_P);

    let rs = reduce_ct_simd(alloc, asm, s);

    u260_to_u256_simd(alloc, asm, rs)
}

fn u260_to_u256_simd(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
    limbs: [Reg<Simd<u64, 2>>; 5],
) -> [Reg<Simd<u64, 2>>; 4] {
    let [l0, l1, l2, l3, l4] = limbs;

    let shifted_l1 = ushr2d(alloc, asm, &l1, 12);
    let shifted_l2 = ushr2d(alloc, asm, &l2, 24);
    let shifted_l3 = ushr2d(alloc, asm, &l3, 36);

    [
        sli2d(alloc, asm, l0, &l1, 52),
        sli2d(alloc, asm, shifted_l1, &l2, 40),
        sli2d(alloc, asm, shifted_l2, &l3, 28),
        sli2d(alloc, asm, shifted_l3, &l4, 16),
    ]
}

/// Does reduction with -2p, but DOESN'T return a clean 52 bit limbs.
/// It doesn't clean up the carries in the upper 52bit. u260-to-u256 takes care of that.
/// This allows us to drop 5 vector instructions.
fn reduce_ct_simd(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
    red: [Reg<Simd<u64, 2>>; 6],
) -> [Reg<Simd<u64, 2>>; 5] {
    // Set cmp to zero if the msb (4x52 + 47) is set.
    let msb_mask = mov(alloc, asm, 1 << 47);
    let msb_mask = dup2d(alloc, asm, &msb_mask);
    let msb = and16(alloc, asm, &red[5], &msb_mask);
    // The comparison state is stored in a vector register instead of NCVF
    // Therefore these operations can be interleaved without making it atomic
    let cmp = cmeq2d(alloc, asm, &msb, 0);

    let subtrahend: [Reg<Simd<_, 2>>; 5] = U52_2P.map(|i| {
        let p = load_const_simd(alloc, asm, i);
        // p & (~cmp) -> if msb is set return p else return 0
        bic16(alloc, asm, &p, &cmp)
    });

    let mut c = array::from_fn(|_| alloc.fresh());
    let [prev, minuend @ ..] = red;
    let mut prev = prev.as_();

    for i in 0..c.len() {
        let tmp = sub2d(alloc, asm, minuend[i].as_(), &subtrahend[i].as_());
        // tmp + (prev >> 52)
        let tmp_plus_borrow = ssra2d(alloc, asm, tmp, &prev, 52);
        c[i] = tmp_plus_borrow;
        prev = &c[i];
    }

    c.map(|ci| ci.into_())
}
