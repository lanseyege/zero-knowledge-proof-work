use rust_proof::bullet_proof_prover::BulletProofProver;
use rust_proof::bullet_proof_prover::BulletProofProverImpl;

fn main() {
    let config_name : &str = "conf/config.toml";
    let store_name : &str = "data/keys_hex_file.txt";
    let mut prover = BulletProofProver::new();
    prover.set_pubs(config_name, store_name);
    let store_name1 : &str = "data/bf_prover_to_bf_verifier.txt";
    //let store_name2 : &str = "data/vecs_to_verifier_file.txt";
    let store_name2 : &str = "data/bf_prover_to_ip_prover.txt";
    prover.range_proof_prover(store_name1, store_name2);
}


