use rust_proof::bullet_proof_verifier::BulletProofVerifierImpl;
use rust_proof::bullet_proof_verifier::BulletProofVerifier;

fn main() {
    let config_name : &str = "conf/config.toml";
    let keys_name : &str = "data/keys_hex_file.txt";
    let mut verifier = BulletProofVerifier::new();
    verifier.set_pubs(config_name, keys_name);
    let store_name : &str = "data/bf_prover_to_bf_verifier.txt";
    let pass_name : &str = "data/bf_verifier_to_ip_verifier.txt";
    verifier.range_proof_verifier(store_name, pass_name);
}

