use bellman::Circuit;
use ff::PrimeField;
use pairing::bn256::Bn256;
use pairing::bn256::Fr;
use sapling_crypto::circuit::test::TestConstraintSystem;

use circuit::GeneratedCircuit;

fn main() {
    let mut system = TestConstraintSystem::<Bn256>::new();

    let circuit = GeneratedCircuit {
        result: Fr::from_str("16").unwrap(),
    };
    circuit.synthesize(&mut system).expect("Synthesis failed");

    dbg!(system.find_unconstrained());
    dbg!(system.num_constraints());
    dbg!(system.num_inputs());

    if let Some(token) = system.which_is_unsatisfied() {
        eprintln!("Error: {} is unsatisfied", token);
    }
}
