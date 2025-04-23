use std::collections::BTreeSet;

use crate::{BasicRegister, HardwareRegister, InstructionF, RegisterMapping, ReifiedRegister};

pub fn generate_standalone_asm(
    label: &str,
    instructions: &Vec<InstructionF<HardwareRegister>>,
) -> String {
    let mut asm_code = String::new();
    let label = format!("_{label}");
    asm_code.push_str(&format!(".global {label}\n.align 4\n.text\n"));
    asm_code.push_str(&format!("{label}:\n"));
    asm_code.extend(
        instructions
            .into_iter()
            .map(|instruction| format!("  {}\n", instruction)),
    );
    asm_code.push_str("ret\n");
    asm_code
}

pub fn format_instructions_rust_inline(
    instructions: &Vec<InstructionF<HardwareRegister>>,
) -> String {
    let mut asm_code = String::new();
    asm_code.extend(
        instructions
            .into_iter()
            .map(|instruction| format!("\"{}\",\n", instruction)),
    );
    asm_code
}

pub fn generate_rust_inline_asm(
    mapping: RegisterMapping,
    inputs_registers: Vec<Vec<ReifiedRegister<HardwareRegister>>>,
    outputs_registers: Vec<Vec<ReifiedRegister<HardwareRegister>>>,
    instructions: &Vec<InstructionF<HardwareRegister>>,
) -> String {
    let inst = format_instructions_rust_inline(instructions);
    let operands =
        generate_asm_operands(mapping, inputs_registers, outputs_registers, instructions);

    let mut asm_code = String::new();
    asm_code.push_str("asm!(\n");
    asm_code.push_str(&inst);
    asm_code.push_str(",\n");
    asm_code.push_str(&operands);
    asm_code.push_str("\n);");
    asm_code
}

// Should make use of basic register
pub fn generate_asm_operands(
    mapping: RegisterMapping,
    inputs_registers: Vec<Vec<ReifiedRegister<HardwareRegister>>>,
    outputs_registers: Vec<Vec<ReifiedRegister<HardwareRegister>>>,
    instructions: &Vec<InstructionF<HardwareRegister>>,
) -> String {
    assert_eq!(
        mapping.allocated(),
        outputs_registers
            .iter()
            .map(|output_register| output_register.len())
            .sum()
    );

    let inputs: Vec<_> = inputs_registers
        .iter()
        .enumerate()
        .flat_map(|(n, input_register)| {
            input_register
                .iter()
                .enumerate()
                .map(move |(i, r)| format!("in(\"{}\") in{n}[{i}]", r.to_basic_register()))
        })
        .intersperse(", ".to_string())
        .collect();

    // Make this work with
    let outputs: Vec<_> = outputs_registers
        .iter()
        .enumerate()
        .flat_map(|(n, output_registers)| {
            output_registers
                .iter()
                .enumerate()
                .map(move |(i, r)| format!("lateout(\"{}\") out{n}[{i}]", r.to_basic_register()))
        })
        .intersperse(", ".to_string())
        .collect();

    let mut clobber_registers: BTreeSet<BasicRegister> = BTreeSet::new();
    instructions.iter().for_each(|instruction| {
        clobber_registers.extend(
            instruction
                .extract_registers()
                .map(|reg| reg.to_basic_register()),
        );
    });

    let output_registers = BTreeSet::from_iter(
        outputs_registers
            .into_iter()
            .flat_map(|r| r.into_iter().map(|reg| reg.to_basic_register())),
    );

    // For the clobbers
    let clobbers = clobber_registers
        .difference(&output_registers)
        .map(|r| format!("lateout(\"{}\") _", r))
        .intersperse(", ".to_string());

    let newline = std::iter::once(",\n".to_string());
    // We jump to the assembly code with br so we need to safe the lr register
    // This can change in the future
    let lr = std::iter::once("lateout(\"lr\") _".to_string());

    inputs
        .into_iter()
        .chain(newline.clone())
        .chain(outputs)
        .chain(newline.clone())
        .chain(clobbers)
        .chain(newline.clone())
        .chain(lr)
        .collect()
}
