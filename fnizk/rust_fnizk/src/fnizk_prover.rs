#![allow(non_snake_case)]
use rug::{Assign, Integer};
use rug::rand::RandState;
use sha256::digest;

use crate::utils::{read_variables, get_config, get_sys_time_in_secs, write_to_file, commit_s, commit_v, get_t1, get_t2, get_lx, get_rx, inner_product, get_vec_hi};

pub trait FnizkProverImpl {
    fn set_pubs(&mut self, config_name : &str, store_name : &str);
    fn set_func(&mut self);
    fn fnizk_prover(&mut self, store_name1 : &str, store_name2 : &str) ;
    fn write_to_files(&mut self, store_name1 : &str, gamma_vec : &mut Vec<Integer>, t1_vec : &mut Vec<Integer>, t2_vec : &mut Vec<Integer>, tau1_vec : &mut Vec<Integer>, tau2_vec : &mut Vec<Integer>);
    fn pass_to_inner_product_prover(&mut self, store_name2 : &str, _random_x : Integer, _random_y : Integer, _random_z : Integer);
}

pub struct FnizkProver<'a> {
    bit_length : usize,
    p_length : u32,
    witness : u32,
    prime_reps : u32,
    is_random : u32,
    rand_seed : u32,
    func_nums : usize,

    _p : Integer,
    _q : Integer,
    _g : Integer,
    _h : Integer,
    _u : Integer,
    _vec_g : Vec<Integer>,
    _vec_h : Vec<Integer>,
    rand : RandState<'a>,
    
    //scalar to vector now
    _p_up : Vec<Integer>,
    _c : Vec<Integer>,
    
    _v_up : Vec<Integer>,
    _a_up : Vec<Integer>, 
    _s_up : Vec<Integer>,
    _t1_up : Vec<Integer>,
    _t2_up : Vec<Integer>,

    _vec_a : Vec<Vec<Integer>>,
    _vec_b : Vec<Vec<Integer>>,

    _hat_t : Vec<Integer>,
    _tau_x : Vec<Integer>,
    _mu : Vec<Integer>,

    witnesses : Vec<u32>,
    bit_lengths : Vec<usize>,
    func_pk : Vec<Integer>,
}

impl<'a> FnizkProver<'_> {
    pub fn new() -> FnizkProver<'a> {
        FnizkProver {
            bit_length : 0,
            p_length : 0,
            witness : 0,
            prime_reps : 0,
            is_random : 0,
            rand_seed : 0,
            func_nums : 0,

            _p : Integer::new(),
            _q : Integer::new(),
            _g : Integer::new(),
            _h : Integer::new(),
            _u : Integer::new(),

            _vec_g : Vec::<Integer>::new(),
            _vec_h : Vec::<Integer>::new(),
            rand : RandState::new(),

            // scalar to vector now
            _p_up : Vec::<Integer>::new(),
            _c : Vec::<Integer>::new(),
            
            _v_up : Vec::<Integer>::new(),
            _a_up : Vec::<Integer>::new(),
            _s_up : Vec::<Integer>::new(),
            _t1_up : Vec::<Integer>::new(),
            _t2_up : Vec::<Integer>::new(),

            _vec_a : Vec::new(),
            _vec_b : Vec::new(),

            _hat_t : Vec::<Integer>::new(),
            _tau_x : Vec::<Integer>::new(),
            _mu : Vec::<Integer>::new(),

            witnesses : Vec::<u32>::new(),
            bit_lengths : Vec::<usize>::new(),
            func_pk : Vec::<Integer>::new(),
        }
    }
}

impl FnizkProverImpl for FnizkProver<'_> {

    fn set_pubs(&mut self, config_name : &str, store_name : &str) {
        (self.bit_length, self.p_length, self.witness, self.prime_reps, self.is_random, self.rand_seed, self.func_nums) = get_config(config_name) ;
        let mut seed = Integer::new();
        if self.is_random == 0 {
            seed.assign(self.rand_seed);
        } 
        else {
            seed.assign(get_sys_time_in_secs());
        }
        self.rand.seed(&seed);
        read_variables(store_name, &mut self._p, &mut self._q, &mut self._g, &mut self._h, &mut self._u, &mut self._vec_g, &mut self._vec_h, &mut self.func_pk);
        self.set_func();
    }
    fn set_func(&mut self ) {
        let mut _bit_len = self.bit_length;
        self.bit_lengths.push(_bit_len);
        self.witnesses.push(self.witness);
        let mut func_a = Vec::<u32>::new();
        for _i in 0..self.func_nums {
            _bit_len /= 2;
            self.bit_lengths.push(_bit_len);
            let temp = Integer::from(Integer::i_pow_u(2, _bit_len as u32));
            //let temp = 2.pow(_bit_len);
            let mut temp2 = temp.clone().random_below(&mut self.rand).to_u32_wrapping();
            let mut a = self.witness - temp2; 
            //println!("temp {}\n temp2 {}", temp, temp2);
            loop {
                if a < self.witness && a > 0 {
                    func_a.push(a);
                    //println!("a {}", a);
                    break;
                } else {
                    temp2 = temp.clone().random_below(&mut self.rand).to_u32_wrapping();
                    a = self.witness - temp2;
                }
            }
        }
        for i in 0..self.func_nums {
            self.witnesses.push(self.witness - func_a[i]);
        }
        //println!("lengths: {:?}", self.bit_lengths);
        //println!("witnesses: {:?}", self.witnesses);
    }
    //fn fnizk_prover_before_yz(&mut self) {
    fn fnizk_prover(&mut self, store_name1 : &str, store_name2 : &str) {
        let mut al_vec =  Vec::new();
        let mut ar_vec =  Vec::new();
        let mut SL_vec =  Vec::new();
        let mut SR_vec =  Vec::new();
        let mut gamma_vec = Vec::<Integer>::new();
        let mut alpha_vec = Vec::<Integer>::new();
        let mut rho_vec = Vec::<Integer>::new();
        let mut tau1_vec = Vec::<Integer>::new();
        let mut tau2_vec = Vec::<Integer>::new();
        let mut t1_vec = Vec::<Integer>::new();
        let mut t2_vec = Vec::<Integer>::new();
        for _l in 0..(self.func_nums + 1) {
            al_vec.push(Vec::<Integer>::new());
            ar_vec.push(Vec::<Integer>::new());
            SL_vec.push(Vec::<Integer>::new());
            SR_vec.push(Vec::<Integer>::new());
        }
        for iter in 0..(self.func_nums + 1) {
            let lens = self.bit_lengths[iter];
            for _i in 0..lens {
                let mut _prime1 = self._p.clone().random_below(&mut self.rand);
                let mut _prime2 = self._p.clone().random_below(&mut self.rand);
                SL_vec[iter].push(_prime1);
                SR_vec[iter].push(_prime2);
            }
            gamma_vec.push(self._p.clone().random_below(&mut self.rand));
            alpha_vec.push(self._p.clone().random_below(&mut self.rand));
            rho_vec.push(self._p.clone().random_below(&mut self.rand));
            tau1_vec.push(self._p.clone().random_below(&mut self.rand));
            tau2_vec.push(self._p.clone().random_below(&mut self.rand));
            let witness_int = Integer::from(self.witnesses[iter]);
            let mut _witness = format!("{0:b}", witness_int);
            _witness = _witness.chars().rev().collect::<String>();
            let temp_len = _witness.len();
            if temp_len < lens {
                let constant = lens - temp_len;
                let mut ss = String::new();
                for _i in 0..constant { ss.push('0'); }
                _witness = format!("{}{}", _witness, ss);
            }
            for c in _witness.chars() {
                let temp = Integer::from(c as u32 - 48);
                ar_vec[iter].push((temp.clone()-1+self._p.clone())%self._p.clone());
                al_vec[iter].push(temp);
            }
            println!("witness_int: {}", witness_int);
            println!("ar_vec: {:?}", ar_vec);
            println!("al_vec: {:?}", al_vec);
            let mut temp_v_up = Integer::new();
            let mut temp_a_up = Integer::new();
            let mut temp_s_up = Integer::new();
            commit_s(self._g.clone(), &witness_int, self._h.clone(), &gamma_vec[iter], self._q.clone(), &mut temp_v_up);
            self._v_up.push(temp_v_up);
            commit_v(&self._vec_g, &al_vec[iter], &self._vec_h, &ar_vec[iter], lens, self._q.clone(), &mut temp_a_up);
            let temp1 = self._h.clone().pow_mod(&alpha_vec[iter], &self._q).unwrap();
            self._a_up.push(temp1 * temp_a_up % self._q.clone());
            commit_v(&self._vec_g, &SL_vec[iter], &self._vec_h, &SR_vec[iter], lens, self._q.clone(), &mut temp_s_up);
            let temp2 = self._h.clone().pow_mod(&rho_vec[iter], &self._q).unwrap();
            self._s_up.push(temp2 * temp_s_up%self._q.clone());
        }
        println!("alpha_vec: {:?}", alpha_vec);
        println!("rho_vec: {:?}", rho_vec);
        println!("SL_vec: {:?}", SL_vec);
        println!("SR_vec: {:?}", SR_vec);
        let mut y_str = String::new();
        for iter in 0..(self.func_nums+1) {
            y_str.push_str(&format!("{}{}", self._a_up[iter].clone(), self._s_up[iter].clone()));
        }
        let _random_y = Integer::from_str_radix(&digest(y_str.clone()), 16).unwrap() % self._p.clone();
        let z_str = format!("{}{}", y_str, _random_y.clone());
        let _random_z = Integer::from_str_radix(&digest(z_str.clone()), 16).unwrap() % self._p.clone();
        for iter in 0..(self.func_nums + 1) {
            let lens = self.bit_lengths[iter];
            let mut t1 = Integer::new();
            let mut t2 = Integer::new();
            get_t1(&al_vec[iter], &ar_vec[iter], &SL_vec[iter], &SR_vec[iter], &_random_y, &_random_z, lens, &self._p, &mut t1);
            get_t2(&SL_vec[iter], &SR_vec[iter], &_random_y, lens, &self._p, &mut t2);
            t1_vec.push(t1.clone());
            t2_vec.push(t2.clone());
            let mut temp_t1_up = Integer::new();
            let mut temp_t2_up = Integer::new();
            if iter == 0 {
                let temp_h_pk = self._h.clone().pow_mod(&Integer::from(self.func_nums), &self._q).unwrap() * self.func_pk[0].clone();
                commit_s(self._g.clone(), &(self.func_nums * t1.clone()), temp_h_pk.clone(), &tau1_vec[0].clone(), self._q.clone(), &mut temp_t1_up);
                commit_s(self._g.clone(), &(self.func_nums * t2.clone()), temp_h_pk.clone(), &tau2_vec[0].clone(), self._q.clone(), &mut temp_t2_up);
            } else {
                let temp_h_pk = self._h.clone() * self.func_pk[iter].clone();
                commit_s(self._g.clone(), &t1, temp_h_pk.clone(), &tau1_vec[iter].clone(), self._q.clone(), &mut temp_t1_up);
                commit_s(self._g.clone(), &t2, temp_h_pk.clone(), &tau2_vec[iter].clone(), self._q.clone(), &mut temp_t2_up);
            }
            self._t1_up.push(temp_t1_up);
            self._t2_up.push(temp_t2_up);
        }
        let mut x_str = String::new();
        for iter in 0..(self.func_nums + 1) {
            x_str.push_str(&format!("{}{}", self._t1_up[iter].clone(), self._t2_up[iter].clone()));
        }
        let _random_x = Integer::from_str_radix(&digest(x_str), 16).unwrap() % self._p.clone(); 
        for iter in 0..(self.func_nums + 1) {
            let lens = self.bit_lengths[iter];
            let mut temp_vec_a = Vec::<Integer>::new();
            let mut temp_vec_b = Vec::<Integer>::new();
            let mut temp_hat_t = Integer::new();
            get_lx(&al_vec[iter], &SL_vec[iter], &_random_x,  &_random_z, lens, &self._p, &mut temp_vec_a);
            get_rx(&ar_vec[iter], &SR_vec[iter], &_random_x, &_random_y, &_random_z, lens, &self._p, &mut temp_vec_b);
            self._vec_a.push(temp_vec_a.clone());
            self._vec_b.push(temp_vec_b.clone());
            inner_product(&temp_vec_a, 0, &temp_vec_b, 0, lens, &self._p, &mut temp_hat_t);
            self._hat_t.push(temp_hat_t);
            self._tau_x.push((tau2_vec[iter].clone() * _random_x.clone() * _random_x.clone() + tau1_vec[iter].clone() * _random_x.clone() + _random_z.clone() * _random_z.clone() * gamma_vec[iter].clone()) % self._p.clone()) ;
            self._mu.push((alpha_vec[iter].clone() + rho_vec[iter].clone() * _random_x.clone()) % self._p.clone());
        }
        println!("_hat_t: {:?}", self._hat_t);
        self.write_to_files(store_name1, &mut gamma_vec, &mut t1_vec, &mut t2_vec, &mut tau1_vec, &mut tau2_vec);
        self.pass_to_inner_product_prover(store_name2, _random_x.clone(), _random_y.clone(), _random_z.clone());
        println!("randomx y z : {};{};{}", _random_x, _random_y, _random_z);
    }
    fn write_to_files(&mut self, store_name1 : &str, gamma_vec : &mut Vec<Integer>, t1_vec : &mut Vec<Integer>, t2_vec : &mut Vec<Integer>, tau1_vec : &mut Vec<Integer>, tau2_vec : &mut Vec<Integer>) {

        let mut content = String::new();
        content.push_str(&format!("_tau_x "));
        for iter in 0..(self.func_nums + 1) {
            content.push_str(&format!("{};", self._tau_x[iter]));
        }
        content.push_str(&format!("\n_mu "));
        for iter in 0..(self.func_nums + 1) {
            content.push_str(&format!("{};", self._mu[iter]));
        }
        content.push_str(&format!("\n_hat_t "));
        for iter in 0..(self.func_nums + 1) {
            content.push_str(&format!("{};", self._hat_t[iter]));
        }
        content.push_str(&format!("\n_v_up "));
        for iter in 0..(self.func_nums + 1) {
            content.push_str(&format!("{};", self._v_up[iter]));
        }
        content.push_str(&format!("\n_a_up "));
        for iter in 0..(self.func_nums + 1) {
            content.push_str(&format!("{};", self._a_up[iter]));
        }
        content.push_str(&format!("\n_s_up "));
        for iter in 0..(self.func_nums + 1) {
            content.push_str(&format!("{};", self._s_up[iter]));
        }
        content.push_str(&format!("\n_t1_up "));
        for iter in 0..(self.func_nums + 1) {
            content.push_str(&format!("{};", self._t1_up[iter]));
        }
        content.push_str(&format!("\n_t2_up "));
        for iter in 0..(self.func_nums + 1) {
            content.push_str(&format!("{};", self._t2_up[iter]));
        }
        content.push_str(&format!("\ngamma_vec "));
        for iter in 0..(self.func_nums + 1) {
            content.push_str(&format!("{};", gamma_vec[iter]));
        }
        content.push_str(&format!("\nt1_vec "));
        for iter in 0..(self.func_nums + 1) {
            content.push_str(&format!("{};", t1_vec[iter]));
        }
        content.push_str(&format!("\nt2_vec "));
        for iter in 0..(self.func_nums + 1) {
            content.push_str(&format!("{};", t2_vec[iter]));
        }
        content.push_str(&format!("\ntau1_vec "));
        for iter in 0..(self.func_nums + 1) {
            content.push_str(&format!("{};", tau1_vec[iter]));
        }
        content.push_str(&format!("\ntau2_vec "));
        for iter in 0..(self.func_nums + 1) {
            content.push_str(&format!("{};", tau2_vec[iter]));
        }
        write_to_file(&store_name1, &mut content);
    }
    fn pass_to_inner_product_prover(&mut self, store_name2 : &str, _random_x : Integer, _random_y : Integer, _random_z : Integer) {
        for iter in 0..(self.func_nums + 1) {
            let lens = self.bit_lengths[iter];
            let mut _vec_hi = Vec::<Integer>::new();
            get_vec_hi(&self._vec_h, &_random_y, lens, &self._p, &self._q, &mut _vec_hi);
            let mut tempk = Integer::new();
            commit_v(&self._vec_g, &self._vec_a[iter], &_vec_hi, &self._vec_b[iter], lens, self._q.clone(), &mut tempk);
            let p_r = self._h.clone().pow_mod(&self._mu[iter], &self._q).unwrap() * tempk % self._q.clone();
            let mut _temp_p_up = Integer::new();
            _temp_p_up = self._h.clone().pow_mod(&(-1 * self._mu[iter].clone()), &self._q).unwrap() * p_r % self._q.clone();
            let mut content = String::new() ;
            content.push_str(&format!("_p_up {}\n",_temp_p_up));
            content.push_str(&format!("_c {}\n", self._hat_t[iter].clone()));
            content.push_str("_vec_h ");
            for hi in _vec_hi.iter() {
                content.push_str(&format!("{};", hi));
            }
            content.push_str("\n_vec_a ");
            for temp in self._vec_a[iter].clone().iter() {
                content.push_str(&format!("{};", temp));
            }
            content.push_str("\n_vec_b ");
            for temp in self._vec_b[iter].clone().iter() {
                content.push_str(&format!("{};", temp));
            }
            write_to_file(&format!("{}_{}", store_name2, iter), &mut content);
        }
    }
}


