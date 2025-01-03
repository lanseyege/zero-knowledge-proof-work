use rug::{Assign, Integer};
use rug::integer::IsPrime;
use rug::rand::RandState;

//mod utils;
use crate::utils::{get_config, get_sys_time_in_secs, write_to_file, generator_yield};

pub fn key_distribution(config_name : &str, store_name : &str) {
    let (bit_length , p_length , _witness, prime_reps, is_random, rand_seed) = get_config(config_name) ;
    
    let mut _p = Integer::new();
    let mut _q = Integer::new();
    let mut _g = Integer::new();
    let mut _h = Integer::new();
    let mut _u = Integer::new();
    let mut _vec_g = Vec::<Integer>::new();
    let mut _vec_h = Vec::<Integer>::new();
    let mut rand = RandState::new();
    let mut seed = Integer::new();
    if is_random == 0 {
        seed.assign(rand_seed);
    } else {
        seed.assign(get_sys_time_in_secs());
    }
    rand.seed(&seed);
    _p.assign(Integer::random_bits(p_length, &mut rand));
    _p.next_prime_mut();
    _q = _p.clone() * 2 + 1;
    while _q.is_probably_prime(prime_reps) == IsPrime::No {
        _p.next_prime_mut();
        _q = _p.clone() * 2 + 1;
    }
    generator_yield(&mut _g, &mut rand, p_length, &mut _p, &mut _q );
    generator_yield(&mut _h, &mut rand, p_length, &mut _p, &mut _q );
    generator_yield(&mut _u, &mut rand, p_length, &mut _p, &mut _q );
    for _i in 0..bit_length {
        let mut _prime1 = Integer::new();
        let mut _prime2 = Integer::new();
        generator_yield(&mut _prime1, &mut rand, p_length, &mut _p, &mut _q );
        generator_yield(&mut _prime2, &mut rand, p_length, &mut _p, &mut _q );
        _vec_g.push(_prime1);
        _vec_h.push(_prime2);
    }

    let mut hex_str = String::new();
    hex_str.push_str("p ");
    hex_str.push_str(&format!("{}", _p));
    hex_str.push_str("\n");
    hex_str.push_str("q ");
    hex_str.push_str(&format!("{}", _q));
    hex_str.push_str("\n");
    hex_str.push_str("g ");
    hex_str.push_str(&format!("{}", _g));
    hex_str.push_str("\n");
    hex_str.push_str("h ");
    hex_str.push_str(&format!("{}", _h));
    hex_str.push_str("\n");
    hex_str.push_str("u ");
    hex_str.push_str(&format!("{}", _u));
    hex_str.push_str("\n");
    hex_str.push_str("vec_g ");
    for temp in _vec_g.iter().map(|x| format!("{}", x)) {
        hex_str.push_str(&(temp + ";"));
    }
    hex_str.push_str("\nvec_h ");
    for temp in _vec_h.iter().map(|x| format!("{}", x)) {
        hex_str.push_str(&(temp + ";"));
    }
    println!("hex: {}" , hex_str);
    write_to_file(store_name , &hex_str);
}


