

cargo build

cargo run --bin key_distribution 
    input file: conf/config.toml
    output file: data/keys_hex_file.txt
    
cargo run --bin bullet_proof_prover
    input file: conf/config.toml 
                data/keys_hex_file.txt

    output file: data/bf_prover_to_ip_prover.txt
                 data/bf_prover_to_bf_verifier.txt

cargo run --bin bullet_proof_verifier
    input file: conf/config.toml
                data/keys_hex_file.txt
                data/bf_prover_to_bf_verifier.txt

    output file: data/bf_verifier_to_ip_verifier.txt

cargo run --bin inner_product_prover
    input file: conf/config.toml
                data/keys_hex_file.txt
                data/bf_prover_to_ip_prover.txt

    output file: data/ip_prover_to_ip_verifier.txt

cargo run --bin inner_product_verifier
    input file: conf/config.toml
                data/keys_hex_file.txt
                data/ip_prover_to_ip_verifier.txt
                 data/bf_verifier_to_ip_verifier.txt
