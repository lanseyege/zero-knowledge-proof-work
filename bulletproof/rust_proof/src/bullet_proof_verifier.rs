#![allow(non_snake_case)]
use rug::{Assign, Integer};
use rug::rand::RandState;
use sha256::digest;

use crate::utils::{read_variables, read_from_file, get_config, get_sys_time_in_secs, commit_s, get_delta, get_vec_hi, get_vec_temp, write_to_file};


pub trait BulletProofVerifierImpl {
    fn set_pubs(&mut self, config_name : &str, store_name : &str);
    fn get_proof_infos(&mut self, store_name1 : &str, _tau_x : &mut Integer, _mu : &mut Integer, _hat_t : &mut Integer, _v_up : &mut Integer, _a_up : &mut Integer, _s_up : &mut Integer, _t1_up : &mut Integer, _t2_up : &mut Integer);
    fn range_proof_verifier(&mut self, store_name : &str, pass_name : &str);
}

pub struct BulletProofVerifier<'a> {
    bit_length : usize,
    p_length : u32,
    witness : u32,
    prime_reps : u32,
    is_random : u32,
    rand_seed : u32,

    _p : Integer,
    _q : Integer,
    _g : Integer,
    _h : Integer,
    _u : Integer,
    _vec_g : Vec<Integer>,
    _vec_h : Vec<Integer>,
    rand : RandState<'a>,
    
    _p_up : Integer,
}

impl<'a> BulletProofVerifier<'_> {
    pub fn new() -> BulletProofVerifier<'a> {
        BulletProofVerifier {
            bit_length : 0,
            p_length : 0,
            witness : 0,
            prime_reps : 0,
            is_random : 0,
            rand_seed : 0,

            _p : Integer::new(),
            _q : Integer::new(),
            _g : Integer::new(),
            _h : Integer::new(),
            _u : Integer::new(),

            _vec_g : Vec::<Integer>::new(),
            _vec_h : Vec::<Integer>::new(),
            rand : RandState::new(),
    
            _p_up : Integer::new(),
        }
    }
}

impl BulletProofVerifierImpl for BulletProofVerifier<'_> {
    fn set_pubs(&mut self, config_name : &str, store_name : &str) {
        (self.bit_length, self.p_length, self.witness, self.prime_reps, self.is_random, self.rand_seed) = get_config(config_name) ;
        let mut seed = Integer::new();
        if self.is_random == 0 {
            seed.assign(self.rand_seed);
        } 
        else {
            seed.assign(get_sys_time_in_secs());
        }
        self.rand.seed(&seed);
        read_variables(store_name, &mut self._p, &mut self._q, &mut self._g, &mut self._h, &mut self._u, &mut self._vec_g, &mut self._vec_h);

        println!("_p: {}", self._p);
        println!("_p: {0:x}", self._p);
        println!("_q: {}", self._q);
        println!("_q: {0:x}", self._q);
        println!("_g: {}", self._g);
        println!("_h: {}", self._h);
        println!("_u: {}", self._u);
        println!("_vec_g: {:?}", self._vec_g);
        println!("_vec_h: {:?}", self._vec_h);

    }
    fn get_proof_infos(&mut self, store_name1 : &str, _tau_x : &mut Integer, _mu : &mut Integer, _hat_t : &mut Integer, _v_up : &mut Integer, _a_up : &mut Integer, _s_up : &mut Integer, _t1_up : &mut Integer, _t2_up : &mut Integer) {
        let mut content = String::new();
        read_from_file(store_name1, &mut content);
        let result = content.split('\n');
        for res in result {
            if !res.is_empty() {
                let mut res2 = res.split(' ');
                let f1 = res2.next().unwrap();
                let f2 = res2.next().unwrap();
                match f1 {
                    "_tau_x" => _tau_x.assign(Integer::parse_radix(f2, 10).unwrap()),
                    "_mu" => _mu.assign(Integer::parse_radix(f2, 10).unwrap()),
                    "_hat_t" => _hat_t.assign(Integer::parse_radix(f2, 10).unwrap()),
                    "_v_up" => _v_up.assign(Integer::parse_radix(f2, 10).unwrap()),
                    "_a_up" => _a_up.assign(Integer::parse_radix(f2, 10).unwrap()),
                    "_s_up" => _s_up.assign(Integer::parse_radix(f2, 10).unwrap()),
                    "_t1_up" => _t1_up.assign(Integer::parse_radix(f2, 10).unwrap()),
                    "_t2_up" => _t2_up.assign(Integer::parse_radix(f2, 10).unwrap()),
                    _ => println!("something wrong!"),
                }
            }
        }
    }
    fn range_proof_verifier(&mut self, store_name : &str, pass_name : &str) {
        let mut _tau_x = Integer::new();
        let mut _mu = Integer::new();
        let mut _hat_t = Integer::new();
        let mut _v_up = Integer::new();
        let mut _a_up = Integer::new();
        let mut _s_up = Integer::new();
        let mut _t1_up = Integer::new();
        let mut _t2_up = Integer::new();
        self.get_proof_infos(store_name, &mut _tau_x, &mut _mu, &mut _hat_t, &mut _v_up, &mut _a_up, &mut _s_up, &mut _t1_up, &mut _t2_up);
        let lens = self.bit_length;
        let _random_x = Integer::from_str_radix(&digest(format!("{}{}", _t1_up.clone(), _t2_up.clone())), 16).unwrap() % self._p.clone();
        let _random_y = Integer::from_str_radix(&digest(format!("{}{}", _a_up.clone(), _s_up.clone())), 16).unwrap() % self._p.clone();
        let _random_z = Integer::from_str_radix(&digest(format!("{}{}{}", _a_up.clone(), _s_up.clone(), _random_y.clone())), 16).unwrap() % self._p.clone();
        let mut check_tl = Integer::new();
        let mut check_tr = Integer::new();
        commit_s(self._g.clone(), &_hat_t, self._h.clone(), &_tau_x, self._q.clone(), &mut check_tl);
        let mut _delta = Integer::new();
        get_delta(&_random_z, &_random_y, lens, &self._p, &mut _delta);
        let temp1 = match _v_up.clone().pow_mod(&(_random_z.clone() * _random_z.clone()), &self._q) {
            Ok(temp1) => temp1,
            Err(_) => unreachable!(),
        };
        let temp2 = match self._g.clone().pow_mod(&_delta, &self._q) {
            Ok(temp2) => temp2,
            Err(_) => unreachable!(),
        };
        let temp3 = match _t1_up.clone().pow_mod(&_random_x.clone(), &self._q) {
            Ok(temp3) => temp3,
            Err(_) => unreachable!(),
        };
        let temp4 = match _t2_up.clone().pow_mod(&(_random_x.clone()*_random_x.clone()), &self._q) {
            Ok(temp4) => temp4,
            Err(_) => unreachable!(),
        };
        check_tr.assign(temp1 * temp2 * temp3 * temp4 % self._q.clone());
        assert_eq!(check_tl, check_tr);
        let mut _vec_hi = Vec::<Integer>::new();
        get_vec_hi(&self._vec_h, &_random_y, lens, &self._p, &self._q, &mut _vec_hi);
        let mut _vec_temp = Vec::<Integer>::new();
        get_vec_temp(&_random_y, &_random_z, &self._p, lens, &mut _vec_temp);
        println!("bfv _vec_temp: {:?}", _vec_temp.clone());
        let mut gz = Integer::from(1);
        for i in 0..lens {
            let a = match self._vec_g[i].clone().pow_mod(&_random_z, &self._q) {
                Ok(a) => a,
                Err(_) => unreachable!(),
            };
            let b = match a.clone().pow_mod(&Integer::from(-1), &self._q) {
                Ok(b) => b,
                Err(_) => unreachable!(),
            };
            println!("i = {}, b = {}", i, b);
            gz *= b;
        }
        gz %= self._q.clone();
        let mut hp = Integer::from(1);
        for i in 0..lens {
            let a = match _vec_hi[i].clone().pow_mod(&_vec_temp[i], &self._q) {
                Ok(a) => a, 
                Err(_) => unreachable!(),
            };
            hp *= a;
        }
        hp %= self._q.clone();
        println!("bfv hp {}", hp.clone());
        let temp5 = _s_up.pow_mod(&_random_x, &self._q).unwrap();
        let PL = _a_up * temp5 * gz * hp % self._q.clone();
        let a = self._h.clone().pow_mod(&_mu, &self._q).unwrap();
        println!("bfv PL {}", PL.clone());
        println!("bfv a {}", a.clone());
        self._p_up.assign( a.pow_mod(&Integer::from(-1), &self._q).unwrap());
        self._p_up.assign(self._p_up.clone() * PL % self._q.clone());
        println!("bfv _p_up {}", self._p_up.clone());
        for i in 0..lens {
            self._vec_h[i].assign(_vec_hi[i].clone());
        }
        let mut content = String::new();
        content.push_str(&format!("_p_up {}\n", self._p_up));
        content.push_str(&format!("_c {}\n", _hat_t));
        content.push_str("_vec_h ");
        for hi in self._vec_h.iter() {
            content.push_str(&format!("{};", hi));
        }
        write_to_file(&pass_name, &content);
        println!("_random_x {}", _random_x);
        println!("_random_y {}", _random_y);
        println!("_random_z {}", _random_z);
    }
}

