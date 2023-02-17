use soroban_test::{Nebula, Wasm};

const WASM: &Wasm = &Wasm::Release("soroban_hello_world_contract");

#[test]
fn can_deploy() {
    Nebula::with_default(|nebula| {
        let path = &WASM.path();
        let friend = "friend";

        nebula
            .new_cmd("contract")
            .arg("invoke")
            .arg("--wasm")
            .arg(path)
            .args(["--id", "1"])
            .args(["--fn", "hello"])
            .arg("--")
            .args(["--to", friend])
            .assert()
            .stderr("")
            .stdout(format!("[\"Hello\",\"{friend}\"]\n"));
    });
}
