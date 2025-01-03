#![allow(non_snake_case)]

use rug::{Assign, Integer};
//use rug::integer::IsPrime;
use rug::rand::RandState;
use sha256::digest;

use crate::utils::{read_variables, get_config,get_sys_time_in_secs,write_to_file,read_from_file,inner_product, commit_v, get_pugh};


pub trait InnerProductProverImpl {
    fn set_pubs(&mut self, config_name : &str, store_name : &str, pass_name : &str);
    fn inner_product_argument(&mut self, file_name : &str) ;
    fn inner_product_argument_proof(&mut self, iter : usize, _bit_length : usize) ;
    fn get_conf_params(&mut self) ;
    fn get_variables(&mut self) ;
    //fn read_variables(&mut self, file_name : &str);
}

pub struct InnerProductProver<'a> {

    bit_length : usize ,
    p_length : u32 ,
    witness : u32 ,
    prime_reps : u32 ,
    is_random : u32 ,
    rand_seed : u32 ,
    func_nums : usize , 

    _p : Integer,
    _q : Integer,
    _g : Integer,
    _h : Integer,
    _u : Vec<Integer>,
    _vec_g : Vec<Vec<Integer>>,
    _vec_h : Vec<Vec<Integer>>,
    rand : RandState<'a>,

    _p_up : Vec<Integer>,
    _c : Vec<Integer>,
    
    _array_l : Vec<Vec<Integer>>,
    _array_r : Vec<Vec<Integer>>,
    _vec_a : Vec<Vec<Integer>>,
    _vec_b : Vec<Vec<Integer>>,

    func_pk : Vec<Integer>,
}


impl<'a> InnerProductProver<'_> {
    pub fn new() -> InnerProductProver<'a> {
        InnerProductProver {
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
            _u : Vec::<Integer>::new(),

            _vec_g : Vec::new(),
            _vec_h : Vec::new(),
            rand : RandState::new(),

            _p_up : Vec::<Integer>::new(),
            _c : Vec::<Integer>::new(),

            _array_l : Vec::new(),
            _array_r : Vec::new(),
            _vec_a : Vec::new(),
            _vec_b : Vec::new(),

            func_pk : Vec::<Integer>::new(),
        }
    }
}


impl InnerProductProverImpl for InnerProductProver<'_> {
    fn set_pubs(&mut self, config_name : &str, store_name : &str, pass_name : &str) {
        (self.bit_length, self.p_length, self.witness, self.prime_reps, self.is_random, self.rand_seed, self.func_nums) = get_config(config_name) ;
        let mut seed = Integer::new() ; // from(get_sys_time_in_secs());
        if self.is_random == 0 {
            seed.assign(self.rand_seed);
        }
        else {
            seed.assign(get_sys_time_in_secs());
        }
        self.rand.seed(&seed);
        for _iter in 0..(self.func_nums + 1) {
            self._vec_a.push(Vec::<Integer>::new());
            self._vec_b.push(Vec::<Integer>::new());
            self._vec_g.push(Vec::<Integer>::new());
            self._vec_h.push(Vec::<Integer>::new());
        }
        self._u.push(Integer::new());
        read_variables(store_name, &mut self._p, &mut self._q, &mut self._g, &mut self._h, &mut self._u[0], &mut self._vec_g[0], &mut self._vec_h[0], &mut self.func_pk);
        let mut lens = self.bit_length;
        for iter in 1..(self.func_nums + 1) {
            lens /= 2;
            self._vec_g[iter] = self._vec_g[0][..lens].to_vec();
        }
        for iter in 0..(self.func_nums + 1) {
            let mut content = String::new();
            read_from_file(&format!("{}_{}", pass_name,iter), &mut content);
            let result = content.split('\n');
            for res in result {
                if !res.is_empty() {
                    let mut res2 = res.split(' ');
                    let f1 = res2.next().unwrap();
                    let f2 = res2.next().unwrap();
                    match f1 {
                        "_c" => self._c.push(Integer::from(Integer::parse_radix(f2, 10).unwrap())),
                        "_p_up" => self._p_up.push(Integer::from(Integer::parse_radix(f2, 10).unwrap())),
                        "_vec_h" => {
                            //if iter !=0 {continue;}
                            if self._vec_h[iter].len() != 0 {
                                self._vec_h[iter].drain(..);
                            }
                            let f3 = f2.split(';');
                            for _f3 in f3 {
                                if !_f3.is_empty() {
                                    self._vec_h[iter].push(Integer::from(Integer::from_str_radix(_f3, 10).unwrap()));
                                }
                            }
                        },
                        "_vec_a" => {
                            if self._vec_a[iter].len() != 0 {
                                self._vec_a[iter].drain(..);
                            }
                            let f3 = f2.split(';');
                            for _f3 in f3 {
                                if !_f3.is_empty() {
                                    self._vec_a[iter].push(Integer::from_str_radix(_f3, 10).unwrap());
                                }
                            }
                            //self._vec_a.push(_vec_a1);
                        },
                        "_vec_b" => {
                            if self._vec_b[iter].len() != 0 {
                                self._vec_b[iter].drain(..);
                            }
                            let f3 = f2.split(';');
                            for _f3 in f3 {
                                if !_f3.is_empty() {
                                    self._vec_b[iter].push(Integer::from_str_radix(_f3, 10).unwrap());
                                }
                            }
                            //self._vec_b.push(_vec_b1);
                        },
                        _ => println!("something wrong!!!"),
                    }
                }
            }
        }
        for _i in 0..(self.func_nums + 1) {
            self._array_l.push(Vec::<Integer>::new());
            self._array_r.push(Vec::<Integer>::new());
        }
        for _i in 1..(self.func_nums + 1) {
            self._u.push(self._u[0].clone());
        }
        let mut lens = self.bit_length;
        for i in 0..(self.func_nums + 1) {
            //let _logn = (lens as f32).log2().ceil() as usize ;
            let mut temp = Integer::new();
            inner_product(&self._vec_a[i], 0, &self._vec_b[i], 0, lens, &self._p, &mut temp);
            //println!("l*r {}: {}", i, temp);
            lens /= 2 ;
        }
        self.get_variables();
    }
    fn inner_product_argument(&mut self, store_name : &str) {
        let mut _bit_length = self.bit_length;
        for iter in 0..(self.func_nums + 1) {
            let mut random_xx_data = String::new();
            //read_from_file(file_name, &mut random_xx_data);
            get_pugh(&self._p_up[iter], &self._u[iter], &self._vec_g[iter], &self._vec_h[iter], &mut random_xx_data);
            let random_xx = Integer::from_str_radix(&digest(random_xx_data), 16).unwrap();
            let temp = self._u[iter].clone().pow_mod(&(random_xx.clone() * self._c[iter].clone()), &self._q).unwrap();
            self._p_up[iter] = self._p_up[iter].clone() * temp % self._q.clone();
            self._u[iter] = self._u[iter].clone().pow_mod(&random_xx, &self._q).unwrap();
            println!("prover self._p_up: {:?}", self._p_up);
            println!("prover self._u: {:?}", self._u);
            println!("prover self._p_up iter {}, {}", iter, self._p_up[iter]);
            println!("prover self._u iter {}, {}", iter, self._u[iter]);
            self.inner_product_argument_proof(iter, _bit_length);
            _bit_length /= 2 ;
        }
        for iter in 0..(self.func_nums + 1) {
            let mut content = String::new();
            content.push_str("_array_l ");
            for tp in self._array_l[iter].iter() {
                content.push_str(&format!("{};", tp));
            }
            content.push_str("\n_array_r ");
            for tp in self._array_r[iter].iter() {
                content.push_str(&format!("{};", tp));
            }
            content.push_str("\na ");
            content.push_str(&format!("{}", self._vec_a[iter][0]));
            content.push_str("\nb ");
            content.push_str(&format!("{}", self._vec_b[iter][0]));
            write_to_file(&format!("{}_{}", store_name, iter), &content);
        }
    }
    fn inner_product_argument_proof(&mut self, iter : usize, bit_length : usize) {
        let mut _c_l_up = Integer::new();
        let mut _c_r_up = Integer::new();
        let mut _l_up = Integer::new();
        let mut _r_up = Integer::new();
        //let mut _bit_length = self.bit_length;
        //let one = Integer::from(1);
        let mut _bit_length = bit_length ;
        loop {
            if _bit_length == 1 {
                //commit_s(self._vec_g)
                break;
            }
            else {
                _bit_length /= 2;
                inner_product(&self._vec_a[iter], 0, &self._vec_b[iter], _bit_length, _bit_length, &self._p, &mut _c_l_up);
                inner_product(&self._vec_a[iter], _bit_length, &self._vec_b[iter], 0, _bit_length, &self._p, &mut _c_r_up);
                //println!("_c_l_up: {} , {} ", _bit_length, _c_l_up);
                //println!("_c_r_up: {} , {} ", _bit_length, _c_r_up);
                let mut _l_up = Integer::new();
                let mut _r_up = Integer::new();
                commit_v(&(self._vec_g[iter][_bit_length..].to_vec()), &(self._vec_a[iter][.._bit_length].to_vec()), &(self._vec_h[iter][.._bit_length].to_vec()), &(self._vec_b[iter][_bit_length..].to_vec()), _bit_length, self._q.clone(), &mut _l_up);
                _l_up = _l_up.clone() * self._u[iter].clone().pow_mod(&_c_l_up , &self._q).unwrap() % self._q.clone();
                commit_v(&(self._vec_g[iter][.._bit_length].to_vec()), &(self._vec_a[iter][_bit_length..].to_vec()), &(self._vec_h[iter][_bit_length..].to_vec()), &(self._vec_b[iter][.._bit_length].to_vec()), _bit_length, self._q.clone(), &mut _r_up);
                _r_up = _r_up.clone() * self._u[iter].clone().pow_mod(&_c_r_up , &self._q).unwrap() % self._q.clone();
                self._array_l[iter].push(_l_up.clone());
                self._array_r[iter].push(_r_up.clone());
                let _random_xx = Integer::from_str_radix(&digest(format!("{}{}", _l_up, _r_up)), 16).unwrap();
                let xx_inv = _random_xx.clone().pow_mod(&Integer::from(-1), &self._p).unwrap();
                let mut _temp_gl = Vec::<Integer>::new();
                let mut _temp_gr = Vec::<Integer>::new();
                for i in 0.._bit_length {
                    _temp_gl.push(self._vec_g[iter][i].clone().pow_mod(&xx_inv, &self._q).unwrap());
                    _temp_gr.push(self._vec_g[iter][i+_bit_length].clone().pow_mod(&_random_xx, &self._q).unwrap());
                }
                self._vec_g[iter].drain(..);
                for i in 0.._bit_length {
                    self._vec_g[iter].push(_temp_gl[i].clone() * _temp_gr[i].clone() % self._q.clone());
                }
                let mut _temp_hl = Vec::<Integer>::new();
                let mut _temp_hr = Vec::<Integer>::new();
                for i in 0.._bit_length {
                    _temp_hl.push(self._vec_h[iter][i].clone().pow_mod(&_random_xx, &self._q).unwrap());
                    _temp_hr.push(self._vec_h[iter][i+_bit_length].clone().pow_mod(&xx_inv, &self._q).unwrap());
                }
                self._vec_h[iter].drain(..);
                for i in 0.._bit_length {
                    self._vec_h[iter].push(_temp_hl[i].clone() * _temp_hr[i].clone() % self._q.clone());
                }
                let xx_sqr = _random_xx.clone() * _random_xx.clone() % self._p.clone();
                let xx_sqr_inv = xx_sqr.clone().pow_mod(&Integer::from(-1), &self._p).unwrap();
                self._p_up[iter] = _l_up.pow_mod(&xx_sqr, &self._q).unwrap() * self._p_up[iter].clone() * _r_up.pow_mod(&xx_sqr_inv, &self._q).unwrap() % self._q.clone();
                for i in 0.._bit_length {
                    self._vec_a[iter][i] = (self._vec_a[iter][i].clone() * _random_xx.clone() + self._vec_a[iter][i+_bit_length].clone() * xx_inv.clone()) % self._p.clone();
                    self._vec_b[iter][i] = (self._vec_b[iter][i].clone() * xx_inv.clone() + self._vec_b[iter][i+_bit_length].clone() * _random_xx.clone()) % self._p.clone();
                }
            }
        }
    }
    fn get_conf_params(&mut self) {
        println!("_bit_length: {}", self.bit_length);
        println!("_p_length: {}", self.p_length);
        println!("witness: {}", self.witness);
        println!("prime_reps: {}", self.prime_reps);
    }
    fn get_variables(&mut self) {
        println!("_p: {}", self._p);
        println!("_q: {}", self._q);
        println!("_g: {}", self._g);
        println!("_h: {}", self._h);
        println!("_u: {:?}", self._u);
        println!("_vec_g: {:?}", self._vec_g.clone());
        println!("_vec_h: {:?}", self._vec_h.clone());
        println!("_vec_a: {:?}", self._vec_a.clone());
        println!("_vec_b: {:?}", self._vec_b.clone());
    }
}



