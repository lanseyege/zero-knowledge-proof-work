use rug::{Assign, Integer};
use rug::integer::IsPrime;
use rug::rand::RandState;

use crate::utils::{get_config, get_sys_time_in_secs, write_to_file, generator_yield};

pub fn key_distribution(config_name : &str, store_name : &str, store_name2 : &str) {
    let (bit_length , p_length , _witness, prime_reps, is_random, rand_seed, func_nums) = get_config(config_name) ;
    
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

    let mut func_sk =  Vec::<Integer>::new();
    let mut func_pk =  Vec::<Integer>::new();
    for _i in 0..(func_nums) {
        func_sk.push(_p.clone().random_below(&mut rand));
    }
    for i in 0..func_nums {
        let temp_i = func_sk[i].clone().pow_mod(&Integer::from(-1) , &_p).unwrap();
        let temp_pk = _h.clone().pow_mod(&temp_i, &_q).unwrap();
        func_pk.push(temp_pk);
    }
    let mut temp_pk = Integer::from(1);
    for i in 0..func_nums {
        temp_pk *= func_pk[i].clone();
    }
    temp_pk %= _q.clone();
    func_pk.insert(0, temp_pk);

    let mut hex_str = String::new();
    hex_str.push_str(&format!("p {}\n", _p));
    hex_str.push_str(&format!("q {}\n", _q));
    hex_str.push_str(&format!("g {}\n", _g));
    hex_str.push_str(&format!("h {}\n", _h));
    hex_str.push_str(&format!("u {}\n", _u));
    hex_str.push_str("vec_g ");
    for temp in _vec_g.iter().map(|x| format!("{}", x)) {
        hex_str.push_str(&(temp + ";"));
    }
    hex_str.push_str("\nvec_h ");
    for temp in _vec_h.iter().map(|x| format!("{}", x)) {
        hex_str.push_str(&(temp + ";"));
    }
    hex_str.push_str("\nfunc_pk ");
    for temp in func_pk.iter() {
        hex_str.push_str(&format!("{};", temp));
    }
    //println!("hex: {}" , hex_str);
    write_to_file(store_name , &hex_str);
    let mut hex_str2 = String::new();
    hex_str2.push_str("func_sk ");
    for temp in func_sk.iter() {
        hex_str2.push_str(&format!("{};", temp));
    }
    write_to_file(store_name2, &hex_str2);
}


