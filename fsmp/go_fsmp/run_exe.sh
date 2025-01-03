
./fSMP ./config/params.toml ./config/witness.toml

./get_commitment ./config/witness.toml ./keys/crs_new ./config/f.param ./keys/pedersen ./keys/r_com

./fSMP_prover ./config/params.toml ./config/witness.toml

./fSMP_verifier ./config/params.toml 
