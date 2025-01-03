
use rug::{Integer, Assign};
use rug::rand::RandState;
use sha256::digest;

use crate::utils::{read_variables, read_from_file, get_config, get_sys_time_in_secs, commit_s, get_pugh};

pub trait InnerProductVerifierImpl {
    fn set_pubs(&mut self, config_name : &str, store_name : &str, pass_name1 : &str, pass_name2 : &str) ;
    fn inner_product_argument_verify(&mut self) ;
    fn print_some_variables(&mut self) ;
}
    
pub struct InnerProductVerifier<'a> {
    bit_length : usize ,
    p_length : u32 ,
    witness : u32 ,
    prime_reps : u32 ,
    is_random : u32 ,
    rand_seed : u32 ,

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
    
    _array_l : Vec<Integer>,
    _array_r : Vec<Integer>,

    _vec_a0 : Integer,
    _vec_b0 : Integer,
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

            _array_l : Vec::<Integer>::new(),
            _array_r : Vec::<Integer>::new(),

            _vec_a0 : Integer::new(),
            _vec_b0 : Integer::new(),
        }
    }
}

impl InnerProductVerifierImpl for InnerProductVerifier<'_> {
    fn set_pubs(&mut self, config_name : &str, keys_name : &str, pass_name1 : &str, pass_name2 : &str) {
        (self.bit_length, self.p_length, self.witness, self.prime_reps, self.is_random, self.rand_seed) = get_config(config_name) ;
        let mut seed = Integer::new();
        if self.is_random == 0 {
            seed.assign(self.rand_seed);
        } 
        else {
            seed.assign(get_sys_time_in_secs());
        }
        self.rand.seed(&seed);
        read_variables(keys_name, &mut self._p, &mut self._q, &mut self._g, &mut self._h, &mut self._u, &mut self._vec_g, &mut self._vec_h);
        let mut content = String::new();
        read_from_file(&pass_name1, &mut content);
        let mut content2 = String::new();
        read_from_file(&pass_name2, &mut content2);
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
                    "_p_up" => self._p_up.assign(Integer::parse_radix(f2, 10).unwrap()),
                    "_c" => self._c.assign(Integer::parse_radix(f2, 10).unwrap()),
                    "a" => self._vec_a0.assign(Integer::parse_radix(f2, 10).unwrap()),
                    "b" => self._vec_b0.assign(Integer::parse_radix(f2, 10).unwrap()),
                    "_vec_h" => {
                        if self._vec_h.len() != 0 {
                            self._vec_h.drain(..);
                        }
                        let f3 = f2.split(';');
                        for _f3 in f3 {
                            if ! _f3.is_empty() {
                                self._vec_h.push(Integer::from_str_radix(_f3, 10).unwrap());
                            }
                        }
                    },
                    "_array_l" => {
                        if self._array_l.len() != 0 {
                            self._array_l.drain(..);
                        }
                        let f3 = f2.split(';');
                        for _f3 in f3 {
                            if !_f3.is_empty() {
                                self._array_l.push(Integer::from_str_radix(_f3, 10).unwrap());
                            }
                        }
                    },
                    "_array_r" => {
                        if self._array_r.len() != 0 {
                            self._array_r.drain(..);
                        }
                        let f3 = f2.split(';');
                        for _f3 in f3 {
                            if !_f3.is_empty() {
                                self._array_r.push(Integer::from_str_radix(_f3, 10).unwrap());
                            }
                        }
                    },
                    _ => unreachable!(),
                }
            }
        }
        self.print_some_variables();
    }
    fn inner_product_argument_verify(&mut self) {
        let mut _bit_length = self.bit_length;
        let mut random_xx_data = String::new();
        get_pugh(&self._p_up, &self._u, &self._vec_g, &self._vec_h, &mut random_xx_data);
        let random_xx = Integer::from_str_radix(&digest(random_xx_data), 16).unwrap();
        let temp = self._u.clone().pow_mod(&(random_xx.clone() * self._c.clone()), &self._q).unwrap();
        self._p_up.assign(self._p_up.clone() * temp % self._q.clone());
        self._u.assign(self._u.clone().pow_mod(&random_xx, &self._q).unwrap());
        let _logn=(_bit_length as f32).log2().ceil() as usize ;
        for i in 0.._logn {
            _bit_length /= 2 ;
            let _random_xx = Integer::from_str_radix(&digest(format!("{}{}", self._array_l[i], self._array_r[i])), 16).unwrap();
            let xx_inv = _random_xx.clone().pow_mod(&Integer::from(-1), &self._p).unwrap();
            let mut _temp_gl = Vec::<Integer>::new();
            let mut _temp_gr = Vec::<Integer>::new();
            for j in 0.._bit_length {
                _temp_gl.push(self._vec_g[j].clone().pow_mod(&xx_inv, &self._q).unwrap());
                _temp_gr.push(self._vec_g[j+_bit_length].clone().pow_mod(&_random_xx, &self._q).unwrap());
            }
            self._vec_g.drain(..);
            for j in 0.._bit_length {
                self._vec_g.push(_temp_gl[j].clone() * _temp_gr[j].clone() % self._q.clone());
            }
            let mut _temp_hl = Vec::<Integer>::new();
            let mut _temp_hr = Vec::<Integer>::new();
            for j in 0.._bit_length {
                _temp_hl.push(self._vec_h[j].clone().pow_mod(&_random_xx, &self._q).unwrap());
                _temp_hr.push(self._vec_h[j+_bit_length].clone().pow_mod(&xx_inv, &self._q).unwrap());
            }
            self._vec_h.drain(..);
            for j in 0.._bit_length {
                self._vec_h.push(_temp_hl[j].clone() * _temp_hr[j].clone() % self._q.clone());
            }
            let xx_sqr = _random_xx.clone() * _random_xx.clone() % self._p.clone();
            let xx_sqr_inv = xx_sqr.clone().pow_mod(&Integer::from(-1), &self._p).unwrap();
            self._p_up.assign(self._array_l[i].clone().pow_mod(&xx_sqr, &self._q).unwrap() * self._p_up.clone() * self._array_r[i].clone().pow_mod(&xx_sqr_inv, &self._q).unwrap() % self._q.clone());
        }
        let temp1 = self._u.clone().pow_mod(&(self._vec_a0.clone() * self._vec_b0.clone() % self._p.clone()), &self._q).unwrap() ;
        let mut temp2 = Integer::new();
        commit_s(self._vec_g[0].clone(), &self._vec_a0, self._vec_h[0].clone(), &self._vec_b0, self._q.clone(), &mut temp2);
        let _p_up_prime = temp1 * temp2 % self._q.clone();
        assert_eq!(self._p_up, _p_up_prime);
    } 
    fn print_some_variables(&mut self) {

        println!("_p_up: {}", self._p_up.clone());
        println!("_c: {}", self._c.clone());
        println!("_vec_a0: {}", self._vec_a0.clone());
        println!("_vec_b0: {}", self._vec_b0.clone());
        println!("_array_l: {:?}", self._array_l.clone());
        println!("_array_r: {:?}", self._array_r.clone());
        println!("_vec_h: {:?}", self._vec_h.clone());
    }
}
