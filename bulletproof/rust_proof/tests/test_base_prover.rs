use rust_proof::inner_product_prover::BaseProverImpl;
use rust_proof::inner_product_prover::BaseProver;

#[test]
fn test1() {
    let file_name : &str = "conf/config.toml";
    let mut test_base = BaseProver::new();
    test_base.set_pubs(file_name);
    test_base.set_generators();
    test_base.get_variables();
    test_base.get_conf_params();
}
