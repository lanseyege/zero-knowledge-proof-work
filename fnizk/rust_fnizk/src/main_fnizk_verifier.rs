
use rust_fnizk::fnizk_verifier::FnizkVerifier;
use rust_fnizk::fnizk_verifier::FnizkVerifierImpl;

fn main() {
    let config_name : &str = "conf/config.toml";
    let keys_name : &str = "data/fnizk_keys.txt";
    let secret_keys_name : &str = "data/fnizk_secret_keys.txt";
    let mut verifier = FnizkVerifier::new();
    verifier.set_pubs(config_name, keys_name, secret_keys_name);

    let store_name : &str = "data/fnizk_prover_to_fnizk_verifier.txt";
    let pass_name : &str = "data/fnizk_verifier_to_ip_verifier.txt";
    verifier.fnizk_verifier(store_name, pass_name);
}

