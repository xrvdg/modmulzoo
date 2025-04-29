use std::array;

use hla::*;

// TODO don't rely on montgomery_reduction for anything other than tests
// Possible not even then
use montgomery_reduction::yuval::{U64_2P, U64_I1, U64_I2, U64_I3, U64_MU0, U64_P};

use crate::load_store::load_const;

/* BUILDERS */

pub fn setup_widening_mul_u256(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
) -> (Vec<FreshVariable>, FreshVariable) {
    let a = alloc.fresh_array();
    let b = alloc.fresh_array();

    let s = widening_mul_u256(alloc, asm, &a, &b);

    (
        vec![FreshVariable::new("a", &a), FreshVariable::new("b", &b)],
        FreshVariable::new("out", &s),
    )
}

pub fn setup_montgomery(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
) -> (Vec<FreshVariable>, FreshVariable) {
    let a = alloc.fresh_array();
    let b = alloc.fresh_array();

    let s = montgomery(alloc, asm, &a, &b);
    (
        vec![FreshVariable::new("a", &a), FreshVariable::new("b", &b)],
        FreshVariable::new("out", &s),
    )
}

pub fn setup_madd_u256_limb(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
) -> (Vec<FreshVariable>, FreshVariable) {
    let add = alloc.fresh_array();
    let var_add = FreshVariable::new("r#add", &add);
    let a = alloc.fresh_array();
    let b = alloc.fresh();

    let s = madd_u256_limb(alloc, asm, add, &a, &b);

    (
        vec![
            var_add,
            FreshVariable::new("a", &a),
            FreshVariable::new("b", &[b]),
        ],
        FreshVariable::new("out", &s),
    )
}

/* GENERATORS */

pub fn carry_add(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
    s: &[Reg<u64>; 2],
    add: &Reg<u64>,
) -> [Reg<u64>; 2] {
    let ret = array::from_fn(|_| alloc.fresh());
    asm.append_instruction(vec![
        adds_inst(&ret[0], &s[0], add),
        cinc_inst(&ret[1], &s[1], "hs".to_string()),
    ]);
    ret
}

pub fn carry_cmn(asm: &mut Assembler, s: [Reg<u64>; 2], add: &Reg<u64>) -> Reg<u64> {
    asm.append_instruction(vec![
        cmn_inst(&s[0], add),
        cinc_inst(&s[1], &s[1], "hs".to_string()),
    ]);
    let [_, out] = s;
    out
}

pub fn madd_u256_limb(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
    mut t: [Reg<u64>; 5],
    a: &[Reg<u64>; 4],
    b: &Reg<u64>,
) -> [Reg<u64>; 5] {
    let mut carry;
    // first multiplication of a carry chain doesn't have a carry to add,
    // but it does have a value already from a previous round
    let tmp = widening_mul(alloc, asm, &a[0], &b);
    [t[0], carry] = carry_add(alloc, asm, &tmp, &t[0]);
    for i in 1..a.len() {
        let tmp = widening_mul(alloc, asm, &a[i], &b);
        let tmp = carry_add(alloc, asm, &tmp, &carry);
        [t[i], carry] = carry_add(alloc, asm, &tmp, &t[i]);
    }
    t[a.len()] = add(alloc, asm, &t[a.len()], &carry);

    t
}

// There is an add truncate to satisfy the assembler
// using smult_add would result in an instruction that gives a
// source that isn't used
pub fn madd_u256_limb_truncate(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
    mut t: [Reg<u64>; 5],
    a: &[Reg<u64>; 4],
    b: &Reg<u64>,
) -> [Reg<u64>; 4] {
    // Allocates unnecessary fresh registers

    // first multiplication of a carry chain doesn't have a carry to add,
    // but it does have a value already from a previous round
    let tmp = widening_mul(alloc, asm, &a[0], &b);
    let mut carry = carry_cmn(asm, tmp, &t[0]);
    for i in 1..a.len() {
        let tmp = widening_mul(alloc, asm, &a[i], &b);
        let tmp = carry_add(alloc, asm, &tmp, &carry);
        [t[i], carry] = carry_add(alloc, asm, &tmp, &t[i]);
    }
    t[a.len()] = add(alloc, asm, &t[a.len()], &carry);

    let [_, out @ ..] = t;
    out
}

pub fn widening_mul_u256(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
    a: &[Reg<u64>; 4],
    b: &[Reg<u64>; 4],
) -> [Reg<u64>; 8] {
    let mut t: [Reg<u64>; 8] = array::from_fn(|_| alloc.fresh());
    let mut carry;
    // The first carry chain is separated out as t doesn't have any values to add
    // first multiplication of a carry chain doesn't not have a carry to add
    [t[0], carry] = widening_mul(alloc, asm, &a[0], &b[0]);
    for i in 1..a.len() {
        let tmp = widening_mul(alloc, asm, &a[i], &b[0]);
        [t[i], carry] = carry_add(alloc, asm, &tmp, &carry);
    }
    t[a.len()] = carry;

    // 2nd and later carry chain
    for j in 1..b.len() {
        let mut carry;
        // first multiplication of a carry chain doesn't have a carry to add,
        // but it does have a value already from a previous round
        let tmp = widening_mul(alloc, asm, &a[0], &b[j]);
        [t[j], carry] = carry_add(alloc, asm, &tmp, &t[j]);
        for i in 1..a.len() {
            let tmp = widening_mul(alloc, asm, &a[i], &b[j]);
            let tmp = carry_add(alloc, asm, &tmp, &carry);
            [t[i + j], carry] = carry_add(alloc, asm, &tmp, &t[i + j]);
        }
        t[j + a.len()] = carry;
    }

    t
}

pub fn sub_u256(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
    a: &[Reg<u64>; 4],
    b: &[Reg<u64>; 4],
) -> [Reg<u64>; 4] {
    let out = array::from_fn(|_| alloc.fresh());
    // Due to carry chain this needs to be an atomic block.
    asm.append_instruction(vec![
        subs_inst(&out[0], &a[0], &b[0]),
        sbcs_inst(&out[1], &a[1], &b[1]),
        sbcs_inst(&out[2], &a[2], &b[2]),
        sbcs_inst(&out[3], &a[3], &b[3]),
    ]);
    out
}

// Reduce within 256-2p
pub fn reduce(alloc: &mut FreshAllocator, asm: &mut Assembler, a: [Reg<u64>; 4]) -> [Reg<u64>; 4] {
    let p2 = U64_2P.map(|val| load_const(alloc, asm, val));
    let red = sub_u256(alloc, asm, &a, &p2);
    let out = array::from_fn(|_| alloc.fresh());
    asm.append_instruction(vec![
        tst_inst(&a[3], 1 << 63),
        csel_inst(&out[0], &red[0], &a[0], "mi"),
        csel_inst(&out[1], &red[1], &a[1], "mi"),
        csel_inst(&out[2], &red[2], &a[2], "mi"),
        csel_inst(&out[3], &red[3], &a[3], "mi"),
    ]);
    out
}

pub fn montgomery(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
    a: &[Reg<u64>; 4],
    b: &[Reg<u64>; 4],
) -> [Reg<u64>; 4] {
    let t = widening_mul_u256(alloc, asm, a, b);
    // let [t0, t1, t2, s @ ..] = t;
    let [t0, t1, t2, s @ ..] = t;

    let i3 = U64_I3.map(|val| load_const(alloc, asm, val));
    let r1 = madd_u256_limb(alloc, asm, s, &i3, &t0);

    let i2 = U64_I2.map(|val| load_const(alloc, asm, val));
    let r2 = madd_u256_limb(alloc, asm, r1, &i2, &t1);

    let i1 = U64_I1.map(|val| load_const(alloc, asm, val));
    let r3 = madd_u256_limb(alloc, asm, r2, &i1, &t2);

    let mu0 = load_const(alloc, asm, U64_MU0);
    let m = mul(alloc, asm, &mu0, &r3[0]);

    let p = U64_P.map(|val| load_const(alloc, asm, val));
    let r4 = madd_u256_limb_truncate(alloc, asm, r3, &p, &m);

    reduce(alloc, asm, r4)
}

pub fn widening_mul(
    alloc: &mut FreshAllocator,
    asm: &mut Assembler,
    a: &Reg<u64>,
    b: &Reg<u64>,
) -> [Reg<u64>; 2] {
    [mul(alloc, asm, a, b), umulh(alloc, asm, a, b)]
}

pub mod experiments {
    use crate::load_store::{load_const, load_u256, store_u256};

    use super::*;

    pub fn setup_single_step_load(
        alloc: &mut FreshAllocator,
        asm: &mut Assembler,
    ) -> (Vec<FreshVariable>, FreshVariable) {
        let mut a = alloc.fresh();
        let b = alloc.fresh();

        single_step_load(alloc, asm, &mut a, &b);

        let var_a = FreshVariable::new("a", &[a]);

        (vec![var_a.clone(), FreshVariable::new("b", &[b])], var_a)
    }

    pub fn setup_single_step_split(
        alloc: &mut FreshAllocator,
        asm: &mut Assembler,
    ) -> (Vec<FreshVariable>, FreshVariable) {
        let a = alloc.fresh_array();
        let b = alloc.fresh_array();

        let s = single_step_split(alloc, asm, &a, &b);
        (
            vec![FreshVariable::new("a", &a), FreshVariable::new("b", &b)],
            FreshVariable::new("out", &s),
        )
    }

    pub fn smult(
        alloc: &mut FreshAllocator,
        asm: &mut Assembler,
        a: [Reg<u64>; 4],
        b: Reg<u64>,
    ) -> [Reg<u64>; 5] {
        // Allocates unnecessary fresh registers
        let mut t: [Reg<u64>; 5] = array::from_fn(|_| alloc.fresh());
        // Ouside of the loop because there is no carry add for the left most dword
        [t[0], t[1]] = widening_mul(alloc, asm, &a[0], &b);
        for i in 1..a.len() {
            let lohi = widening_mul(alloc, asm, &a[i], &b);
            [t[i], t[i + 1]] = carry_add(alloc, asm, &lohi, &t[i]);
        }

        t
    }

    // TODO better name
    pub fn school_method_load(
        alloc: &mut FreshAllocator,
        asm: &mut Assembler,
        a: &Reg<*const [u64; 4]>,
        b: &Reg<*const [u64; 4]>,
    ) -> [Reg<u64>; 8] {
        let mut t: [Reg<u64>; 8] = array::from_fn(|_| alloc.fresh());
        let mut carry;
        // The first carry chain is separated out as t doesn't have any values to add
        // first multiplication of a carry chain doesn't not have a carry to add
        let mut a_load: [Reg<u64>; 4] = array::from_fn(|_| alloc.fresh());
        let mut b_load: [Reg<u64>; 4] = array::from_fn(|_| alloc.fresh());

        // TODO loading it one-by-one doesn't have an added benefit as it doesn't
        // reduce the maximum number registers that are used over time.
        // So this could be done with ldp
        a_load[0] = ldr(alloc, asm, &a.get(0));
        b_load[0] = ldr(alloc, asm, &b.get(0));

        [t[0], carry] = widening_mul(alloc, asm, &a_load[0], &b_load[0]);
        for i in 1..a_load.len() {
            a_load[i] = ldr(alloc, asm, &a.get(i));
            let tmp = widening_mul(alloc, asm, &a_load[i], &b_load[0]);
            [t[i], carry] = carry_add(alloc, asm, &tmp, &carry);
        }
        t[a_load.len()] = carry;

        // 2nd and later carry chain
        for j in 1..b_load.len() {
            b_load[j] = ldr(alloc, asm, &b.get(j));
            let mut carry;
            // first multiplication of a carry chain doesn't have a carry to add,
            // but it does have a value already from a previous round
            let tmp = widening_mul(alloc, asm, &a_load[0], &b_load[j]);
            [t[j], carry] = carry_add(alloc, asm, &tmp, &t[j]);
            for i in 1..a_load.len() {
                let tmp = widening_mul(alloc, asm, &a_load[i], &b_load[j]);
                let tmp = carry_add(alloc, asm, &tmp, &carry);
                [t[i + j], carry] = carry_add(alloc, asm, &tmp, &t[i + j]);
            }
            t[j + a_load.len()] = carry;
        }

        t
    }

    pub fn single_step_load<'a>(
        alloc: &mut FreshAllocator,
        asm: &mut Assembler,
        a: &Reg<*mut [u64; 4]>,
        b: &Reg<*const [u64; 4]>,
    ) {
        let load_a = load_u256(alloc, asm, a.as_());
        let b = load_u256(alloc, asm, b);

        let res = montgomery(alloc, asm, &load_a, &b);

        store_u256(alloc, asm, &res, a);
    }

    pub fn addv(
        alloc: &mut FreshAllocator,
        asm: &mut Assembler,
        a: [Reg<u64>; 5],
        b: [Reg<u64>; 5],
    ) -> [Reg<u64>; 5] {
        let t: [Reg<u64>; 5] = array::from_fn(|_| alloc.fresh());
        let n: usize = t.len();

        let mut instructions = Vec::new();
        instructions.push(adds_inst(&t[0], &a[0], &b[0]));
        for i in 1..n - 1 {
            instructions.push(adcs_inst(&t[i], &a[i], &b[i]));
        }
        instructions.push(adc_inst(&t[n - 1], &a[n - 1], &b[n - 1]));
        asm.append_instruction(instructions);

        t
    }

    pub fn addv_truncate(
        alloc: &mut FreshAllocator,
        asm: &mut Assembler,
        a: [Reg<u64>; 5],
        b: [Reg<u64>; 5],
    ) -> [Reg<u64>; 4] {
        let t: [Reg<u64>; 4] = array::from_fn(|_| alloc.fresh());

        let mut instructions = Vec::new();
        instructions.push(cmn_inst(&a[0], &b[0]));
        for i in 1..a.len() {
            instructions.push(adcs_inst(&t[i - 1], &a[i], &b[i]));
        }
        instructions.push(adc_inst(&t[3], &a[4], &b[4]));
        asm.append_instruction(instructions);

        t
    }

    pub fn single_step_split(
        alloc: &mut FreshAllocator,
        asm: &mut Assembler,
        a: &[Reg<u64>; 4],
        b: &[Reg<u64>; 4],
    ) -> [Reg<u64>; 4] {
        let t = widening_mul_u256(alloc, asm, a, b);
        // let [t0, t1, t2, s @ ..] = t;
        let [t0, t1, t2, s @ ..] = t;

        let i3 = U64_I3.map(|val| load_const(alloc, asm, val));
        let r1 = smult(alloc, asm, i3, t0);

        let i2 = U64_I2.map(|val| load_const(alloc, asm, val));
        let r2 = smult(alloc, asm, i2, t1);

        let i1 = U64_I1.map(|val| load_const(alloc, asm, val));
        let r3 = smult(alloc, asm, i1, t2);

        let r4 = addv(alloc, asm, r1, r2);
        let r5 = addv(alloc, asm, r4, r3);
        let r6 = addv(alloc, asm, r5, s);

        let mu0 = load_const(alloc, asm, U64_MU0);
        let m = mul(alloc, asm, &mu0, &r6[0]);

        let p = U64_P.map(|val| load_const(alloc, asm, val));
        let r7 = smult(alloc, asm, p, m);
        let r8 = addv_truncate(alloc, asm, r7, r6);

        reduce(alloc, asm, r8)
    }
}
