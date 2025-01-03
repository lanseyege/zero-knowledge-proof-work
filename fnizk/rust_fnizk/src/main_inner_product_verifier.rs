
use rust_fnizk::inner_product_verifier::InnerProductVerifier;
use rust_fnizk::inner_product_verifier::InnerProductVerifierImpl;

fn main() {
    let config_name : &str = "conf/config.toml";
    let keys_name : &str = "data/fnizk_keys.txt";
    let secret_keys_name : &str = "data/fnizk_secret_keys.txt";
    let pass_name1 : &str = "data/fnizk_verifier_to_ip_verifier.txt";
    let pass_name2 : &str = "data/ip_prover_to_ip_verifier";
    let mut verifier = InnerProductVerifier::new();
    verifier.set_pubs(config_name, keys_name, secret_keys_name, pass_name1, pass_name2);
    verifier.inner_product_argument_verify();
    println!("knowledge extraction: ");
    verifier.knowledge_extract();
}

