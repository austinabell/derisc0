// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use methods::{METHOD_NAME_ELF, METHOD_NAME_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    let env = ExecutorEnv::builder()
        // Send a & b to the guest
        .write(&3)
        .unwrap()
        .write(&4)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    let receipt = prover.prove_elf(env, METHOD_NAME_ELF).unwrap();

    let c: u32 = receipt.journal.decode().expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );
    assert_eq!(c, 12);

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(METHOD_NAME_ID).unwrap();
}
