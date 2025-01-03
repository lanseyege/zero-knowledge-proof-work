use rust_proof::inner_product_verifier::InnerProductVerifierImpl;
use rust_proof::inner_product_verifier::InnerProductVerifier;

fn main() {

    let config_name : &str= "conf/config.toml";
    let keys_name : &str = "data/keys_hex_file.txt";
    let pass_name1 : &str = "data/bf_verifier_to_ip_verifier.txt";
    let pass_name2 : &str = "data/ip_prover_to_ip_verifier.txt";
    let mut verifier = InnerProductVerifier::new();
    verifier.set_pubs(config_name, keys_name, pass_name1, pass_name2);
    verifier.inner_product_argument_verify();
}
