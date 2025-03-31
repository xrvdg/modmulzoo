use std::array;

use hla::{
    Allocator, Assembler, Reg, RegisterBank, RegisterMapping, Seen, backend_global, backend_rust,
    hardware_register_allocation, input, liveness_analysis,
};

fn main() {}

fn single_step(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    a: [Reg<f64>; 4],
    b: [Reg<f64>; 4],
) -> [Reg<f64>; 4] {
    todo!()
}

fn build_single_step() {
    let mut alloc = Allocator::new();
    let mut mapping = RegisterMapping::new();
    let mut phys_registers = RegisterBank::new();

    let mut asm = Assembler::new();
    let a = array::from_fn(|i| input(&mut alloc, &mut mapping, &mut phys_registers, i as u64));
    let b = array::from_fn(|i| {
        input(
            &mut alloc,
            &mut mapping,
            &mut phys_registers,
            (a.len() + i) as u64,
        )
    });

    let input_hw_registers: Vec<_> = a
        .iter()
        .chain(&b)
        .filter_map(|reg| mapping.output_register(reg))
        .collect();

    let s = single_step(&mut alloc, &mut asm, a, b);

    let first: Vec<_> = asm.instructions.into_iter().flatten().collect();

    // Is there something we can do to tie off the outputs.
    // and to make sure it happens before drop_pass
    let mut seen = Seen::new();
    s.iter().for_each(|r| {
        seen.output_interface(r);
    });

    let releases = liveness_analysis(&mut seen, &first);

    let out = hardware_register_allocation(&mut mapping, &mut phys_registers, first, releases);

    let output_hw_registers: Vec<_> = s
        .iter()
        .filter_map(|reg| mapping.output_register(reg))
        .collect();

    let outputs = backend_rust(mapping, &input_hw_registers, &output_hw_registers, &out);

    let mut file =
        std::fs::File::create("./asm/global_asm_single_step.s").expect("Unable to create file");
    let txt = backend_global("single_step".to_string(), out);

    // Write this info in the assembly file

    println!("{}", outputs);

    use std::io::Write;
    file.write_all(txt.as_bytes())
        .expect("Unable to write data to file");
}
