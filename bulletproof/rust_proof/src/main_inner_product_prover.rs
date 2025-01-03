use rust_proof::inner_product_prover::InnerProductProverImpl;
use rust_proof::inner_product_prover::InnerProductProver;


fn main() {
    let config_name : &str = "conf/config.toml";
    let keys_name : &str = "data/keys_hex_file.txt";
    let pass_name : &str = "data/bf_prover_to_ip_prover.txt";
    let mut prover = InnerProductProver::new();
    prover.set_pubs(config_name, keys_name, pass_name);
    let store_name : &str = "data/ip_prover_to_ip_verifier.txt";
    prover.inner_product_argument(store_name);
}
