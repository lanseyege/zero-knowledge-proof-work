
use rust_proof::bullet_proof_key_generation::key_distribution;

fn main() {
    let config_name : &str = "conf/config.toml";
    let store_name : &str = "data/keys_hex_file.txt";
    //bullet_proof_key_generation::key_distribution(config_name, store_name);
    key_distribution(config_name, store_name);
}
