use algebra::{ 
    PrimeField
};
use r1cs_std::{boolean::Boolean, R1CSVar};
/// Helper used to skip operations which should not be executed when running the
/// trusted setup
pub fn is_setup<F: PrimeField>(message: &[Boolean<F>]) -> bool {
    message.iter().any(|m| m.value().is_err())
}

/// Converts the provided bits to LE bytes
pub fn bits_to_bytes(bits: &[bool]) -> Vec<u8> {
    let reversed_bits = {
        let mut tmp = bits.to_owned();
        tmp.reverse();
        tmp
    };

    let mut bytes = vec![];
    for chunk in reversed_bits.chunks(8) {
        let mut byte = 0;
        let mut twoi: u64 = 1;
        for c in chunk {
            byte += (twoi * (*c as u64)) as u8;
            twoi *= 2;
        }
        bytes.push(byte);
    }

    bytes
}

/// If bytes is a little endian representation of a number, this returns the bits
/// of the number in descending order
pub fn bytes_to_bits(bytes: &[u8], bits_to_take: usize) -> Vec<bool> {
    let mut bits = vec![];
    for b in bytes {
        let mut byte = *b;
        for _ in 0..8 {
            bits.push((byte & 1) == 1);
            byte >>= 1;
        }
    }

    bits.into_iter()
        .take(bits_to_take)
        .collect::<Vec<bool>>()
        .into_iter()
        .rev()
        .collect()
}

#[cfg(any(test, feature = "test-helpers"))]
pub mod test_helpers {
    use algebra::PrimeField;
    use r1cs_core::ConstraintSystemRef;

    pub fn print_unsatisfied_constraints<F: PrimeField>(cs: ConstraintSystemRef<F>) -> () {
       if !cs.is_satisfied().unwrap() {
            println!("=========================================================");
            println!("Unsatisfied constraints:");
            println!("{}", cs.which_is_unsatisfied().unwrap().unwrap());
            println!("=========================================================");
        } 
    }
}
