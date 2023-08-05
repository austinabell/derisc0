// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use methods::{METHOD_NAME_ELF, METHOD_NAME_ID};
use risc0_zkvm::{
    default_executor_from_elf,
    serde::{from_slice, to_vec},
    ExecutorEnv,
};

fn main() {
    let env = ExecutorEnv::builder()
    // Send a & b to the guest
    .add_input(&to_vec(&3).unwrap())
    .add_input(&to_vec(&4).unwrap())
    .build()
    .unwrap();

    // Next, we make an executor, loading the (renamed) ELF binary.
    let mut exec = default_executor_from_elf(env, METHOD_NAME_ELF).unwrap();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    let c: u32 = from_slice(&receipt.journal).expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );
    assert_eq!(c, 12);

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(METHOD_NAME_ID).unwrap();
}
