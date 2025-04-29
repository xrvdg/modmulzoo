use block_multiplier_codegen::scalar::*;
use block_multiplier_codegen::simd::*;

use hla::{
    build,
    builder::{Interleaving, build_single},
};

fn main() {
    // commented out now that it takes a constant
    build_single("./asm/smul_add.s", "smul_add", setup_smul_add);
    build_single("./asm/school_method.s", "school_method", setup_schoolmethod);
    build_single("./asm/single_step.s", "single_step", setup_single_step);
    build_single(
        "./asm/single_step_load.s",
        "single_step_load",
        setup_single_step_load,
    );
    build_single(
        "./asm/single_step_split.s",
        "single_step_split",
        setup_single_step_split,
    );
    build_single(
        "./asm/u256_to_u260_shl2_simd.s",
        "u256_to_u260_shl2_simd",
        setup_u256_to_u260_shl2_imd,
    );
    build_single(
        "./asm/u260_to_u256_simd.s",
        "u260_to_u256_simd",
        setup_u260_to_u256_simd,
    );
    build_single(
        "./asm/vmultadd_noinit_simd.s",
        "vmultadd_noinit_simd",
        setup_vmultadd_noinit_simd,
    );
    build_single(
        "./asm/single_step_simd.s",
        "single_step_simd",
        setup_single_step_simd,
    );
    build_single(
        "./asm/reduce_ct_simd.s",
        "reduce_ct_simd",
        setup_reduce_ct_simd,
    );
    build(
        "./asm/single_step_interleaved.s",
        "single_step_interleaved",
        Interleaving::par(
            Interleaving::single(setup_single_step),
            Interleaving::single(setup_single_step_simd),
        ),
    );
    build(
        "./asm/single_step_interleaved_seq_scalar.s",
        "single_step_interleaved_seq_scalar",
        Interleaving::par(
            Interleaving::seq(vec![setup_single_step, setup_single_step]),
            Interleaving::single(setup_single_step_simd),
        ),
    );
    build(
        "./asm/single_step_interleaved_triple_scalar.s",
        "single_step_interleaved_triple_scalar",
        Interleaving::par(
            Interleaving::seq(vec![
                setup_single_step_load,
                setup_single_step_load,
                setup_single_step_load,
            ]),
            Interleaving::single(setup_single_step_simd),
        ),
    );
}
