use crate::backend::{
    RegisterBank, RegisterMapping, allocate_input_variable, hardware_register_allocation,
    reserve_output_variable,
};
use crate::codegen::generate_rust_global_asm;
use crate::frontend::{Assembler, FreshAllocator, FreshVariable};
use crate::liveness::liveness_analysis;

pub fn build(
    label: &str,
    f: fn(alloc: &mut FreshAllocator, asm: &mut Assembler) -> (Vec<FreshVariable>, FreshVariable),
) {
    let mut alloc = FreshAllocator::new();
    let mut mapping = RegisterMapping::new();
    let mut register_bank = RegisterBank::new();

    let mut asm = Assembler::new();
    let (input_hw_registers, output_hw_register) = f(&mut alloc, &mut asm);

    let output_hw_registers = [output_hw_register];

    let instructions: Vec<_> = asm.instructions.into_iter().flatten().collect();

    // Is there something we n do to tie off the outputs.
    // and to make sure it happens before drop_pass

    let (releases, lifetimes) =
        liveness_analysis(&output_hw_registers, &instructions, alloc.fresh as usize);

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
