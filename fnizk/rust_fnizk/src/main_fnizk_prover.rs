use rust_fnizk::fnizk_prover::FnizkProver;
use rust_fnizk::fnizk_prover::FnizkProverImpl;

fn main() {
    let config_name : &str = "conf/config.toml";
    let store_name : &str = "data/fnizk_keys.txt";
    let mut prover = FnizkProver::new();
    prover.set_pubs(config_name, store_name);
    let store_name1 : &str = "data/fnizk_prover_to_fnizk_verifier.txt";
    let store_name2 : &str = "data/fnizk_prover_to_ip_prover";
    prover.fnizk_prover(store_name1, store_name2);
}
