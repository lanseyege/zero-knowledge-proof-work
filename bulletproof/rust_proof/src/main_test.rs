use rust_proof::inner_product_prover::BaseProverImpl;
use rust_proof::inner_product_prover::BaseProver;

use rug::{Assign, Integer};

fn test_base() {
    let file_name : &str = "conf/config.toml";
    let file_name2 : &str = "data/hex_file.txt";
    let mut test_base = BaseProver::new();
    test_base.set_pubs(file_name);
    test_base.set_generators(file_name2);
    test_base.get_variables();
    test_base.get_conf_params();
    test_base.write_variables(file_name2);
    //test_base.read_variables(file_name2);
    test_base.get_variables();

    let mut vec_a = Vec::<Integer>::new();
    let mut vec_b = Vec::<Integer>::new();
    for i in 0..16 {
        vec_a.push(Integer::from(i)) ;
        vec_b.push(Integer::from(i * 3));
    }

    test_base.inner_product_argument(&vec_a, &vec_b, file_name2);
}

fn test_bullet() {

}

fn main() {
    test_base();
    test_bullet();
}
