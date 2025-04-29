use crate::AtomicInstructionBlock;
use crate::backend::{
    RegisterBank, RegisterMapping, allocate_input_variable, hardware_register_allocation,
    reserve_output_variable,
};
use crate::codegen::generate_rust_global_asm;
use crate::frontend::{Assembler, FreshAllocator, FreshVariable};
use crate::liveness::liveness_analysis;

pub type Setup =
    fn(alloc: &mut FreshAllocator, asm: &mut Assembler) -> (Vec<FreshVariable>, FreshVariable);

pub fn build_single(label: &str, f: Setup) {
    build(label, Interleaving::Seq(vec![f]));
}

pub fn build(label: &str, algos: Interleaving<Setup>) {
    let mut alloc = FreshAllocator::new();
    let mut mapping = RegisterMapping::new();
    let mut register_bank = RegisterBank::new();

    let (input_hw_registers, output_hw_registers, instructions) = run_setups(&mut alloc, algos);

    let instructions: Vec<_> = instructions.into_iter().flatten().collect();

    // Is there something we n do to tie off the outputs.
    // and to make sure it happens before drop_pass

    let (releases, lifetimes) = liveness_analysis(&alloc, &output_hw_registers, &instructions);

    let input_hw_registers = allocate_input_variable(
        &mut mapping,
        &mut register_bank,
        input_hw_registers,
        &lifetimes,
    );

    output_hw_registers.iter().for_each(|variable| {
        reserve_output_variable(&mut register_bank, &lifetimes, variable);
    });

    let out = hardware_register_allocation(
        &mut mapping,
        &mut register_bank,
        instructions,
        releases,
        lifetimes,
    );

    let output_hw_registers: Vec<_> = output_hw_registers
        .iter()
        .map(|fresh_variable| mapping.allocate_variable(fresh_variable))
        .collect();

    // Write this info in the assembly file
    let assembly = generate_rust_global_asm(label, &input_hw_registers, &output_hw_registers, &out);

    use std::io::Write;
    let mut file = std::fs::File::create(format!("./asm/global_asm_{label}.s"))
        .expect("Unable to create file");
    file.write_all(assembly.as_bytes())
        .expect("Unable to write data to file");
}

fn run_setups(
    alloc: &mut FreshAllocator,
    algos: Interleaving<Setup>,
) -> (
    Vec<FreshVariable>, // inputs
    Vec<FreshVariable>, // outputs
    Vec<AtomicInstructionBlock>,
) {
    match algos {
        Interleaving::Seq(items) => items.into_iter().fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |(mut inputs, mut outputs, mut instructions), func| {
                let (input, output, instrs) = run_setup(alloc, func);
                inputs.extend(input);
                outputs.push(output);
                instructions.extend(instrs);
                (inputs, outputs, instructions)
            },
        ),
        Interleaving::Par(left, right) => {
            let (inputs_left, outputs_left, instructions_left) = run_setups(alloc, *left);
            let (inputs_right, outputs_right, instructions_right) = run_setups(alloc, *right);

            let mut inputs = inputs_left;
            inputs.extend(inputs_right);

            let mut outputs = outputs_left;
            outputs.extend(outputs_right);

            let instructions = interleave(instructions_left, instructions_right);

            (inputs, outputs, instructions)
        }
    }
}

fn run_setup(
    alloc: &mut FreshAllocator,
    f: Setup,
) -> (
    Vec<FreshVariable>,
    FreshVariable,
    Vec<AtomicInstructionBlock>,
) {
    let mut asm = Assembler::new();
    let (inputs, outputs) = f(alloc, &mut asm);
    (inputs, outputs, asm.instructions)
}

// This interleaving can be more complex, but we don't need it for the moment and use Seq as a leaf
pub enum Interleaving<T> {
    Seq(Vec<T>),
    Par(Box<Interleaving<T>>, Box<Interleaving<T>>),
}

impl<T> Interleaving<T> {
    pub fn single(t: T) -> Self {
        Interleaving::Seq(vec![t])
    }
    pub fn seq(t: Vec<T>) -> Self {
        Interleaving::Seq(t)
    }
    pub fn par(t1: Interleaving<T>, t2: Interleaving<T>) -> Self {
        Interleaving::Par(Box::new(t1), Box::new(t2))
    }
}

/// Interleaves elements from two vectors.
///
/// This function combines elements from two vectors, distributing the elements
/// from the shorter vector evenly throughout the longer vector.
///
/// # Arguments
///
/// * `lhs` - First vector of elements
/// * `rhs` - Second vector of elements
///
/// # Returns
///
/// A new vector containing all elements from both input vectors, interleaved.
fn interleave<T>(lhs: Vec<T>, rhs: Vec<T>) -> Vec<T> {
    let (shorter, longer) = if lhs.len() <= rhs.len() {
        (lhs, rhs)
    } else {
        (rhs, lhs)
    };

    if shorter.is_empty() {
        return longer;
    }

    let mut result = Vec::with_capacity(shorter.len() + longer.len());

    let short_len = shorter.len();
    let mut short_iter = shorter.into_iter().enumerate();

    let long_len = longer.len();
    let mut long_iter = longer.into_iter();
    // For the first element (short_index = 0 ) -> The location will be ((short_index + 1) * long_len) / short_len
    let mut next = long_len / short_len;

    // With spacing i needs to reach and place the last element of short
    // ((short_len - 1 + 1) * long_len) / short_len = long_len. Therefore the range is 0..=long_len
    for i in 0..=long_len {
        if i == next {
            if let Some((short_index, item)) = short_iter.next() {
                result.push(item);
                // Order is important due to flooring
                // next = index next element (short_index + 1) + 1
                next = ((short_index + 2) * long_len) / short_len;
            }
        }

        if let Some(item) = long_iter.next() {
            result.push(item)
        }
    }

    assert!(short_iter.next().is_none());

    result
}

#[cfg(test)]
mod test {
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn interleave(lhs: Vec<u64>, rhs: Vec<u64>) -> bool {
        let left = lhs.len();
        let right = rhs.len();
        let res = super::interleave(lhs, rhs);
        res.len() == left + right
    }
}
