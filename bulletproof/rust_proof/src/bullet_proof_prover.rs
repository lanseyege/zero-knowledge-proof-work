#![allow(non_snake_case)]
use rug::{Assign, Integer};
use rug::rand::RandState;
use sha256::digest;
use crate::utils::{read_variables, write_to_file, get_config, get_sys_time_in_secs, commit_s, commit_v, get_t1, get_t2, get_lx, get_rx, inner_product, get_vec_hi};

pub trait BulletProofProverImpl {
    fn set_pubs(&mut self, config_name : &str, store_name : &str);
    fn range_proof_prover(&mut self, store_name1 : &str, store_name2 : &str);
    fn write_to_files(&mut self, store_name1 : &str);
    fn pass_to_inner_product_prover(&mut self, store_name2 : &str, _random_x : Integer, _random_y : Integer, _random_z : Integer) ;
}

pub struct BulletProofProver<'a> {
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
    _c : Integer,
    
    _v_up : Integer,
    _a_up : Integer, 
    _s_up : Integer,
    _t1_up : Integer,
    _t2_up : Integer,

    _vec_a : Vec<Integer>,
    _vec_b : Vec<Integer>,

    _hat_t : Integer,
    _tau_x : Integer,
    _mu : Integer,
}

impl<'a> BulletProofProver<'_> {
    pub fn new() -> BulletProofProver<'a> {
        BulletProofProver {
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
            _c : Integer::new(),
            
            _v_up : Integer::new(),
            _a_up : Integer::new(),
            _s_up : Integer::new(),
            _t1_up : Integer::new(),
            _t2_up : Integer::new(),

            _vec_a : Vec::<Integer>::new(),
            _vec_b : Vec::<Integer>::new(),

            _hat_t : Integer::new(),
            _tau_x : Integer::new(),
            _mu : Integer::new(),
        }
    }
}

impl BulletProofProverImpl for BulletProofProver<'_> {
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
    fn range_proof_prover(&mut self, store_name1 : &str, store_name2 : &str ) {
        let mut al = Vec::<Integer>::new(); 
        let mut ar = Vec::<Integer>::new(); 
        let mut SL = Vec::<Integer>::new(); 
        let mut SR = Vec::<Integer>::new(); 
        for _i in 0..self.bit_length {
            let mut _prime1 = self._p.clone().random_below(&mut self.rand); 
            let mut _prime2 = self._p.clone().random_below(&mut self.rand); 
            SL.push(_prime1);
            SR.push(_prime2);
        }
        let mut _gamma = self._p.clone().random_below(&mut self.rand);
        let mut _alpha = self._p.clone().random_below(&mut self.rand);
        let mut _rho = self._p.clone().random_below(&mut self.rand);
        let mut _tau1 = self._p.clone().random_below(&mut self.rand);
        let mut _tau2 = self._p.clone().random_below(&mut self.rand);
        let mut _t1 = Integer::new();
        let mut _t2 = Integer::new();
        let lens = self.bit_length;
        let witness_int = Integer::from(self.witness);
        let mut _witness = format!("{0:b}", self.witness);
        _witness = _witness.chars().rev().collect::<String>();
        let temp_len = _witness.len();
        if temp_len < self.bit_length {
            let constant = self.bit_length - temp_len ;
            let mut ss = String::new();
            for _i in 0..constant {
                ss.push('0');
            }
            _witness = format!("{}{}", _witness, ss);
        } 
        for c in _witness.chars() {
            let temp = Integer::from(c as u32 - 48);
            ar.push((temp.clone() - 1 + self._p.clone() ) % self._p.clone());
            al.push(temp);
        }
        println!("_witness {}", _witness);
        println!("ar {:?}", ar);
        println!("al {:?}", al);
        commit_s(self._g.clone(), &witness_int, self._h.clone(), &_gamma, self._q.clone(), &mut self._v_up);
        commit_v(&self._vec_g, &al, &self._vec_h, &ar, lens, self._q.clone(), &mut self._a_up);
        let temp1 = match self._h.clone().pow_mod(&_alpha, &self._q) {
            Ok(temp1) => temp1,
            Err(_) => unreachable!(),
        };
        self._a_up.assign(temp1 * self._a_up.clone() % self._q.clone());
        commit_v(&self._vec_g, &SL, &self._vec_h, &SR, lens, self._q.clone(), &mut self._s_up);
        let temp2 = match self._h.clone().pow_mod(&_rho, &self._q) {
            Ok(temp2) => temp2,
            Err(_) => unreachable!(),
        };
        self._s_up.assign(temp2 * self._s_up.clone() % self._q.clone());

        let _random_y = Integer::from_str_radix(&digest(format!("{}{}", self._a_up.clone(), self._s_up.clone())), 16).unwrap() % self._p.clone();
        let _random_z = Integer::from_str_radix(&digest(format!("{}{}{}", self._a_up.clone(), self._s_up.clone(), _random_y.clone())), 16).unwrap() % self._p.clone();
        get_t1(&al, &ar, &SL, &SR, &_random_y, &_random_z, lens, &self._p, &mut _t1);
        get_t2(&SL, &SR, &_random_y, lens, &self._p, &mut _t2);
        commit_s(self._g.clone(), &_t1, self._h.clone(), &_tau1.clone(), self._q.clone(), &mut self._t1_up);
        commit_s(self._g.clone(), &_t2, self._h.clone(), &_tau2.clone(), self._q.clone(), &mut self._t2_up);
        let _random_x = Integer::from_str_radix(&digest(format!("{}{}", self._t1_up.clone(), self._t2_up.clone())), 16).unwrap() % self._p.clone();
        get_lx(&al, &SL, &_random_x, &_random_z, lens, &self._p, &mut self._vec_a);
        get_rx(&ar, &SR, &_random_x, &_random_y, &_random_z, lens, &self._p, &mut self._vec_b);
        inner_product(&self._vec_a, 0, &self._vec_b, 0, lens, &self._p, &mut self._hat_t);
        self._tau_x.assign((_tau2 * _random_x.clone() * _random_x.clone() + _tau1 * _random_x.clone() + _random_z.clone() * _random_z.clone() * _gamma) % self._p.clone());
        self._mu.assign((_alpha + _rho * _random_x.clone()) % self._p.clone());

        self.write_to_files(&store_name1 );
        self.pass_to_inner_product_prover(&store_name2, _random_x.clone(), _random_y.clone(), _random_z.clone());

        println!("_v_up : {}", self._v_up.clone());
        println!("_a_up : {}", self._a_up.clone());
        println!("_s_up : {}", self._s_up.clone());
        println!("_random_x : {}", _random_x);
        println!("_random_y : {}", _random_y);
        println!("_random_z : {}", _random_z);
        println!("_t1 : {}", _t1);
        println!("_t2 : {}", _t2);
        println!("_T1 : {}", self._t1_up.clone());
        println!("_T2 : {}", self._t2_up.clone());
        println!("_vec_a : {:?}", self._vec_a.clone());
        println!("_vec_b : {:?}", self._vec_b.clone());
        println!("_hat_t : {}", self._hat_t.clone());
        println!("_tau_x : {}", self._tau_x.clone());
        println!("_mu : {}", self._mu.clone());
    }
    fn write_to_files(&mut self, store_name1 : &str ) {
        //write variables to file
        let mut content = String::new();
        content.push_str(&format!("_tau_x {}\n", self._tau_x.clone()));
        content.push_str(&format!("_mu {}\n", self._mu.clone()));
        content.push_str(&format!("_hat_t {}\n", self._hat_t.clone()));
        content.push_str(&format!("_v_up {}\n", self._v_up.clone()));
        content.push_str(&format!("_a_up {}\n", self._a_up.clone()));
        content.push_str(&format!("_s_up {}\n", self._s_up.clone()));
        content.push_str(&format!("_t1_up {}\n", self._t1_up.clone()));
        content.push_str(&format!("_t2_up {}\n", self._t2_up.clone()));
        write_to_file(&store_name1, &mut content);

    }
    fn pass_to_inner_product_prover(&mut self, store_name2 : &str, _random_x : Integer, _random_y : Integer, _random_z : Integer) {
        //pass _P and _vec_hi to inner_product_prover
        let lens = self.bit_length;
        let mut _vec_hi = Vec::<Integer>::new();
        get_vec_hi(&self._vec_h, &_random_y, lens, &self._p, &self._q, &mut _vec_hi);
        for i in 0..lens {
            self._vec_h[i].assign(_vec_hi[i].clone());
        }
        let mut tempk = Integer::new();
        commit_v(&self._vec_g, &self._vec_a, &self._vec_h, &self._vec_b, lens, self._q.clone(), &mut tempk );
        let p_r = self._h.clone().pow_mod(&self._mu, &self._q).unwrap() * tempk % self._q.clone();
        println!("bfp PR {}", p_r);
        self._p_up = self._h.clone().pow_mod(&(-1 * self._mu.clone()), &self._q).unwrap() * p_r % self._q.clone();
        println!("bfp new _p_up {}", self._p_up);
        let mut content = String::new();
        content.push_str(&format!("_p_up {}\n", self._p_up.clone()));
        content.push_str(&format!("_c {}\n", self._hat_t.clone()));
        content.push_str("_vec_h ");
        for hi in _vec_hi.iter() {
            content.push_str(&format!("{};", hi));
        }
        //write_to_file(&store_name3, &content);
        //write _vec_a, _vec_b to file
        content.push_str("\n_vec_a ");
        for temp in self._vec_a.clone().iter() {
            content.push_str(&format!("{};", temp));
        }
        content.push_str("\n_vec_b ");
        for temp in self._vec_b.clone().iter() {
            content.push_str(&format!("{};", temp));
        }
        write_to_file(&store_name2, &mut content);
    }
}

