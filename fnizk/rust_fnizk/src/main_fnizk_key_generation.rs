
use rust_fnizk::fnizk_key_generation::key_distribution;

fn main() {
    let config_name : &str = "conf/config.toml";
    let store_name : &str = "data/fnizk_keys.txt";
    let store_name2 : &str = "data/fnizk_secret_keys.txt";
    key_distribution(config_name, store_name, store_name2);
}

