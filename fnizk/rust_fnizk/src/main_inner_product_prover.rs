
use rust_fnizk::inner_product_prover::InnerProductProver;
use rust_fnizk::inner_product_prover::InnerProductProverImpl;

fn main() {
    let config_name : &str = "conf/config.toml";
    let keys_name : &str = "data/fnizk_keys.txt";
    let pass_name : &str = "data/fnizk_prover_to_ip_prover";
    let mut prover = InnerProductProver::new();
    prover.set_pubs(config_name, keys_name, pass_name);
    let pass_name2 : &str = "data/ip_prover_to_ip_verifier";
    prover.inner_product_argument(pass_name2);
}


