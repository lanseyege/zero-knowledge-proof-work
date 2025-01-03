
use rug::{Integer, Assign};
use rug::rand::RandState;
use sha256::digest;

use crate::utils::{read_variables, read_from_file, get_config, get_sys_time_in_secs, commit_s, get_pugh};

pub trait InnerProductVerifierImpl {
    fn set_vector(&mut self, temp_v : &mut Vec<Integer>, f2 : &str) ;
    fn set_pubs(&mut self, config_name : &str, keys_name : &str, secret_keys_name : &str, pass_name1 : &str, pass_name2 : &str) ;
    fn inner_product_argument_verify(&mut self) ;
    fn print_some_variables(&mut self) ;
    fn knowledge_extract(&mut self);
}
    
pub struct InnerProductVerifier<'a> {
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

    _vec_a0 : Vec::<Integer>,
    _vec_b0 : Vec::<Integer>,

    func_pk : Vec<Integer>,
    func_sk : Vec<Integer>,
    bit_lengths : Vec<usize>,
    alpha : Vec<Integer>,
    _v_up : Vec<Integer>,
}

impl<'a> InnerProductVerifier<'_> {
    pub fn new() -> InnerProductVerifier<'a> {
        InnerProductVerifier {
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

            _vec_a0 : Vec::<Integer>::new(),
            _vec_b0 : Vec::<Integer>::new(),

            func_pk : Vec::<Integer>::new(),
            func_sk : Vec::<Integer>::new(),
            bit_lengths : Vec::<usize>::new(),
            alpha : Vec::<Integer>::new(),
            _v_up : Vec::<Integer>::new(),
        }
    }
}

impl InnerProductVerifierImpl for InnerProductVerifier<'_> {
    fn set_vector(&mut self, temp_v : &mut Vec<Integer>, f2 : &str) {
        if temp_v.len() != 0 {temp_v.drain(..);}
        let f3 = f2.split(';');
        for _f3 in f3 {
            if ! _f3.is_empty() {
                temp_v.push(Integer::from_str_radix(_f3, 10).unwrap());
            }
        }
    }
    fn set_pubs(&mut self, config_name : &str, keys_name : &str,  secret_keys_name : &str, pass_name1 : &str, pass_name2 : &str) {
        (self.bit_length, self.p_length, self.witness, self.prime_reps, self.is_random, self.rand_seed, self.func_nums) = get_config(config_name) ;
        let mut seed = Integer::new();
        if self.is_random == 0 {
            seed.assign(self.rand_seed);
        }
        else {
            seed.assign(get_sys_time_in_secs());
        }
        self.rand.seed(&seed);
        let mut bit_len = self.bit_length;
        for _iter in 0..(self.func_nums + 1) {
            self._vec_g.push(Vec::<Integer>::new());
            self._vec_h.push(Vec::<Integer>::new());
            self.bit_lengths.push( bit_len);
            bit_len /= 2;
        }
        self._u.push(Integer::new());
        read_variables(keys_name, &mut self._p, &mut self._q, &mut self._g, &mut self._h, &mut self._u[0], &mut self._vec_g[0], &mut self._vec_h[0], &mut self.func_pk);
        for _iter in 0..(self.func_nums + 1) {
            self._array_l.push(Vec::<Integer>::new());
            self._array_r.push(Vec::<Integer>::new());
        }
        let mut content = String::new();
        read_from_file(&pass_name1, &mut content);
        let mut content2 = String::new();
        read_from_file(&secret_keys_name, &mut content2);
        //let mut content = content1 + "\n" + content2;
        content += "\n";
        content += &content2;
        //println!("content: {}", content.clone());
        let result = content.split('\n');
        for res in result {
            if !res.is_empty() {
                let mut res2 = res.split(' ');
                let f1 = res2.next().unwrap();
                let f2 = res2.next().unwrap();
                match f1 {
                    "_p_up" => {
                        let mut temp_v = Vec::<Integer>::new();
                        self.set_vector(&mut temp_v, f2);
                        self._p_up = temp_v
                    }, 
                    "_c" => {
                        let mut temp_v = Vec::<Integer>::new();
                        self.set_vector(&mut temp_v, f2);
                        self._c = temp_v;
                    }, 
                    "_vec_h" => {
                        let mut temp_v = Vec::<Integer>::new();
                        self.set_vector(&mut temp_v, f2);
                        self._vec_h[0] = temp_v;
                    }, 
                    "alpha" => {
                        let mut temp_v = Vec::<Integer>::new();
                        self.set_vector(&mut temp_v, f2);
                        self.alpha = temp_v;
                    },
                    "func_sk" => {
                        let mut temp_v = Vec::<Integer>::new();
                        self.set_vector(&mut temp_v, f2);
                        self.func_sk = temp_v;
                    },
                    "_v_up" => {
                        let mut temp_v = Vec::<Integer>::new();
                        self.set_vector(&mut temp_v, f2);
                        self._v_up = temp_v;
                    },
                    _ => unreachable!(),
                }
            }
        }   
        for iter in 1..(self.func_nums + 1) {
            self._u.push(self._u[0].clone());
            self._vec_h[iter] = self._vec_h[0][..self.bit_lengths[iter]].to_vec();
            self._vec_g[iter] = self._vec_g[0][..self.bit_lengths[iter]].to_vec();
        }
        for iter in 0..(self.func_nums + 1) {
            let mut content = String::new();
            read_from_file(&format!("{}_{}", pass_name2, iter), &mut content);
            let result = content.split('\n');
            for res in result {
                if ! res.is_empty() {
                    let mut res2 = res.split(' ');
                    let f1 = res2.next().unwrap();
                    let f2 = res2.next().unwrap();
                    match f1 {
                        "_array_l" => {
                            let mut temp_v = Vec::<Integer>::new();
                            self.set_vector(&mut temp_v, f2);
                            self._array_l[iter] = temp_v;
                        }, 
                        "_array_r" => {
                            let mut temp_v = Vec::<Integer>::new();
                            self.set_vector(&mut temp_v, f2);
                            self._array_r[iter] = temp_v;
                        }, 
                        "a" => self._vec_a0.push(Integer::from_str_radix(f2, 10).unwrap()),
                        "b" => self._vec_b0.push(Integer::from_str_radix(f2, 10).unwrap()),
                        _ => unreachable!(),
                    }
                }
            }
        }
        self.print_some_variables();
    }
    fn inner_product_argument_verify(&mut self) {
        for iter in 0..(self.func_nums + 1) {
            let mut _bit_length = self.bit_lengths[iter];
            let mut random_xx_data = String::new();
            get_pugh(&self._p_up[iter], &self._u[iter], &self._vec_g[iter], &self._vec_h[iter], &mut random_xx_data);
            let random_xx = Integer::from_str_radix(&digest(random_xx_data), 16).unwrap();
            let temp = self._u[iter].clone().pow_mod(&(random_xx.clone() * self._c[iter].clone()), &self._q).unwrap();
            self._p_up[iter] = self._p_up[iter].clone() * temp % self._q.clone();
            self._u[iter] = self._u[iter].clone().pow_mod(&random_xx, &self._q).unwrap();
            let _logn=(_bit_length as f32).log2().ceil() as usize ;
            //println!("logn : {}", _logn);
            //println!("bit_length: {:?}", self.bit_lengths);
            for i in 0.._logn {
                _bit_length /= 2 ;
                let _random_xx = Integer::from_str_radix(&digest(format!("{}{}", self._array_l[iter][i], self._array_r[iter][i])), 16).unwrap();
                let xx_inv = _random_xx.clone().pow_mod(&Integer::from(-1), &self._p).unwrap();
                let mut _temp_gl = Vec::<Integer>::new();
                let mut _temp_gr = Vec::<Integer>::new();
                for j in 0.._bit_length {
                    _temp_gl.push(self._vec_g[iter][j].clone().pow_mod(&xx_inv, &self._q).unwrap());
                    _temp_gr.push(self._vec_g[iter][j+_bit_length].clone().pow_mod(&_random_xx, &self._q).unwrap());
                }
                self._vec_g[iter].drain(..);
                for j in 0.._bit_length {
                    self._vec_g[iter].push(_temp_gl[j].clone() * _temp_gr[j].clone() % self._q.clone());
                }
                let mut _temp_hl = Vec::<Integer>::new();
                let mut _temp_hr = Vec::<Integer>::new();
                for j in 0.._bit_length {
                    _temp_hl.push(self._vec_h[iter][j].clone().pow_mod(&_random_xx, &self._q).unwrap());
                    _temp_hr.push(self._vec_h[iter][j+_bit_length].clone().pow_mod(&xx_inv, &self._q).unwrap());
                }
                self._vec_h[iter].drain(..);
                for j in 0.._bit_length {
                    self._vec_h[iter].push(_temp_hl[j].clone() * _temp_hr[j].clone() % self._q.clone());
                }
                let xx_sqr = _random_xx.clone() * _random_xx.clone() % self._p.clone();
                let xx_sqr_inv = xx_sqr.clone().pow_mod(&Integer::from(-1), &self._p).unwrap();
                self._p_up[iter] = self._array_l[iter][i].clone().pow_mod(&xx_sqr, &self._q).unwrap() * self._p_up[iter].clone() * self._array_r[iter][i].clone().pow_mod(&xx_sqr_inv, &self._q).unwrap() % self._q.clone();
            }
            let temp1 = self._u[iter].clone().pow_mod(&(self._vec_a0[iter].clone() * self._vec_b0[iter].clone() % self._p.clone()), &self._q).unwrap() ;
            let mut temp2 = Integer::new();
            commit_s(self._vec_g[iter][0].clone(), &self._vec_a0[iter], self._vec_h[iter][0].clone(), &self._vec_b0[iter], self._q.clone(), &mut temp2);
            let _p_up_prime = temp1 * temp2 % self._q.clone();
            assert_eq!(self._p_up[iter], _p_up_prime);
            println!("self._p_up[{}]: {}", iter, self._p_up[iter]);
            println!("_p_up_prime: {}", _p_up_prime);
        }
        println!("Improved Inner-Product Verification PASS!!!");
    }
    fn print_some_variables(&mut self) {
        println!("_p_up: {:?}", self._p_up);
        println!("_c: {:?}", self._c);
        println!("_u: {:?}", self._u);
        println!("_vec_a0: {:?}", self._vec_a0);
        println!("_vec_b0: {:?}", self._vec_b0);
        println!("_array_l: {:?}", self._array_l);
        println!("_array_r: {:?}", self._array_r);
        println!("_vec_h: {:?}", self._vec_h);
        println!("func_sk: {:?}", self.func_sk);
    }
    fn knowledge_extract(&mut self) {
        let base : usize = 2;
        let sizes : usize = base.pow(self.bit_length as u32) as usize - 1;
        //let sizes : usize = Integer::from(2).pow_mod(&Integer::from(self.bit_length), &Integer::from(1)).unwrap().to_usize_wrapping();
        //println!("sizes: {}", sizes);
        for iter in 1..(self.func_nums + 1) {
            let beta = self.alpha[iter-1].clone().pow_mod(&self.func_sk[iter-1], &self._q).unwrap();
            let temp = self._v_up[iter].clone().pow_mod(&Integer::from(-1), &self._q).unwrap();
            let temp2 = beta.pow_mod(&Integer::from(-1), &self._q).unwrap();
            let tempr = temp * self._v_up[0].clone() * temp2 % self._q.clone();
            //println!("tempr: {}", tempr);
            for i in 1..sizes {
                let tempr2 = self._g.clone().pow_mod(&Integer::from(i), &self._q).unwrap();
                if tempr2 == tempr {
                    //let b = Integer::from(2).pow_mod(&Integer::from(self.bit_lengths[iter]-1), &Integer::from(1)).unwrap() + Integer::from(i);
                    let b = base.pow(self.bit_lengths[iter] as u32) as usize -1usize + i as usize;
                    println!("a_{}={}, b_{}={}", iter, i, iter, b); 
                    //i as i64 + 2_i64.pow(self.bit_lengths[iter] as u64)-1
                    break;
                }
            }
        }
    }
}
