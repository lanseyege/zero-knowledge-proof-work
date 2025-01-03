#![allow(non_snake_case)]
use rug::{Assign, Integer};
use rug::rand::RandState;
use sha256::digest;

use crate::utils::{read_variables, get_config, get_sys_time_in_secs, write_to_file, read_from_file, get_vec_hi, get_vec_temp, get_delta};

pub trait FnizkVerifierImpl {
    fn set_pubs(&mut self, config_name : &str, keys_name : &str, secret_keys_name : &str) ;
    fn set_secret_key(&mut self, secret_keys_name : &str) ;
    fn set_proof_infos(&mut self, store_name : &str, _tau_x : &mut Vec<Integer>, _mu : &mut Vec<Integer>, _hat_t : &mut Vec<Integer>, _v_up : &mut Vec<Integer>, _a_up : &mut Vec<Integer>, _s_up : &mut Vec<Integer>, _t1_up : &mut Vec<Integer>, _t2_up : &mut Vec<Integer>, gamma_vec : &mut Vec<Integer>, t1_vec : &mut Vec<Integer>, t2_vec : &mut Vec<Integer>, tau1_vec : &mut Vec<Integer>, tau2_vec : &mut Vec<Integer>) ;
    fn set_vector(&mut self, temp_v : &mut Vec<Integer>, f2 : &str) ;
    fn fnizk_verifier(&mut self, store_name : &str, pass_name : &str) ;
    fn fnizk_synthesis(&mut self, _random_x : &Integer, _random_y : &Integer, _random_z : &Integer, _hat_t : &Vec<Integer>, _tau_x : &Vec<Integer>, gamma_vec : &Vec<Integer>, _v_up : &Vec<Integer>, _s_up : &Vec<Integer>, _t1_up : &Vec<Integer>, _t2_up : &Vec<Integer>, t1_vec : &Vec<Integer>, t2_vec : &Vec<Integer>, tau1_vec : &Vec<Integer>, tau2_vec : &Vec<Integer>);
}

pub struct FnizkVerifier<'a> {
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
    sec_pk : Vec<Integer>,
    func_pk : Vec<Integer>,
}

impl<'a> FnizkVerifier<'_> {
    pub fn new() -> FnizkVerifier<'a> {
        FnizkVerifier {
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
            
            // 
            _p_up : Vec::<Integer>::new(),
            sec_pk : Vec::<Integer>::new(),
            func_pk : Vec::<Integer>::new(),
        }
    }
}

impl FnizkVerifierImpl for FnizkVerifier<'_> {

    fn set_pubs(&mut self, config_name : &str, keys_name : &str, secret_keys_name : &str) {
        (self.bit_length, self.p_length, self.witness, self.prime_reps, self.is_random, self.rand_seed, self.func_nums) = get_config(config_name) ;
        let mut seed = Integer::new();
        if self.is_random == 0 {
            seed.assign(self.rand_seed);
        } else {
            seed.assign(get_sys_time_in_secs());
        }
        self.rand.seed(&seed);
        read_variables(keys_name, &mut self._p, &mut self._q, &mut self._g, &mut self._h, &mut self._u, &mut self._vec_g, &mut self._vec_h, &mut self.func_pk);
        self.set_secret_key(secret_keys_name);
    }
    fn set_secret_key(&mut self, secret_keys_name : &str) {
        let mut content = String::new();
        //let mut file = File::open(secret_keys_name).unwrap();
        //file.read_to_string(&mut content).unwrap();
        read_from_file(secret_keys_name, &mut content);
        let mut res = content.split(' ');
        let f1 = res.next().unwrap();
        let f2 = res.next().unwrap();
        for f3 in f2.split(';') {
            if !f3.is_empty() {
                match f1 {
                    "func_sk" => self.sec_pk.push(Integer::from_str_radix(f3, 10).unwrap()),
                    _ => println!("set secret key wrong!"),
                }
            }
        }
        //println!("sec_pk : {:?}", self.sec_pk);
    }
    fn set_vector(&mut self, temp_v : &mut Vec<Integer>, f2 : &str) {
        if temp_v.len() != 0 {temp_v.drain(..);}
        let f3 = f2.split(';');
        for _f3 in f3 {
            if ! _f3.is_empty() {
                temp_v.push(Integer::from_str_radix(_f3, 10).unwrap());
            }
        }
    }
    fn set_proof_infos(&mut self, store_name : &str, _tau_x : &mut Vec<Integer>, _mu : &mut Vec<Integer>, _hat_t : &mut Vec<Integer>, _v_up : &mut Vec<Integer>, _a_up : &mut Vec<Integer>, _s_up : &mut Vec<Integer>, _t1_up : &mut Vec<Integer>, _t2_up : &mut Vec<Integer>, gamma_vec : &mut Vec<Integer>, t1_vec : &mut Vec<Integer>, t2_vec : &mut Vec<Integer>, tau1_vec : &mut Vec<Integer>, tau2_vec : &mut Vec<Integer>) {
        let mut content = String::new();
        read_from_file(store_name, &mut content);
        let result = content.split('\n');
        for res in result {
            if ! res.is_empty() {
                let mut res2 = res.split(' ');
                let f1 = res2.next().unwrap();
                let f2 = res2.next().unwrap();
                match f1 {
                    "_tau_x" => {
                        self.set_vector(_tau_x, f2);
                    },
                    "_mu" => {
                        self.set_vector(_mu, f2);
                    },
                    "_hat_t" => {
                        self.set_vector(_hat_t, f2);
                    },
                    "_v_up" => {
                        self.set_vector(_v_up, f2);
                    },
                    "_a_up" => {
                        self.set_vector(_a_up, f2);
                    },
                    "_s_up" => {
                        self.set_vector(_s_up, f2);
                    },
                    "_t1_up" => {
                        self.set_vector(_t1_up, f2);
                    },
                    "_t2_up" => {
                        self.set_vector(_t2_up, f2);
                    },
                    "gamma_vec" => {
                        self.set_vector(gamma_vec, f2);
                    },
                    "t1_vec" => {
                        self.set_vector(t1_vec, f2);
                    },
                    "t2_vec" => {
                        self.set_vector(t2_vec, f2);
                    },
                    "tau1_vec" => {
                        self.set_vector(tau1_vec, f2);
                    },
                    "tau2_vec" => {
                        self.set_vector(tau2_vec, f2);
                    },
                    _ => println!("something wrong!!"),
                }
            }
        }
        //println!("_tau_x : {:?}", _tau_x);
        //println!("_t2_up : {:?}", _t2_up);
    }
    fn fnizk_verifier(&mut self, store_name : &str, pass_name : &str) {
        let mut _tau_x = Vec::<Integer>::new();
        let mut _mu = Vec::<Integer>::new();
        let mut _hat_t = Vec::<Integer>::new(); 
        let mut _v_up = Vec::<Integer>::new();
        let mut _a_up = Vec::<Integer>::new();
        let mut _s_up = Vec::<Integer>::new();
        let mut _t1_up = Vec::<Integer>::new();
        let mut _t2_up = Vec::<Integer>::new();
        let mut gamma_vec = Vec::<Integer>::new();
        let mut t1_vec = Vec::<Integer>::new();
        let mut t2_vec = Vec::<Integer>::new();
        let mut tau1_vec = Vec::<Integer>::new();
        let mut tau2_vec = Vec::<Integer>::new();
        self.set_proof_infos(store_name, &mut _tau_x, &mut _mu, &mut _hat_t, &mut _v_up, &mut _a_up, &mut _s_up, &mut _t1_up, &mut _t2_up, &mut gamma_vec, &mut t1_vec, &mut t2_vec, &mut tau1_vec, &mut tau2_vec);
        let mut x_str = String::new();
        let mut y_str = String::new();
        for iter in 0..(self.func_nums + 1) {
            x_str.push_str(&format!("{}{}", _t1_up[iter], _t2_up[iter]));
            y_str.push_str(&format!("{}{}", _a_up[iter], _s_up[iter]));
        }
        let _random_x = Integer::from_str_radix(&digest(x_str), 16).unwrap() % self._p.clone();
        let _random_y = Integer::from_str_radix(&digest(y_str.clone()), 16).unwrap() % self._p.clone();
        let z_str = format!("{}{}", y_str, _random_y.clone());
        let _random_z = Integer::from_str_radix(&digest(z_str), 16).unwrap() % self._p.clone();
        let mut lens = self.bit_length;
        let mut content = String::new();
        let mut _vec_hi2 = Vec::<Integer>::new();
        get_vec_hi(&self._vec_h, &_random_y, lens, &self._p, &self._q, &mut _vec_hi2);
        for iter in 0..(self.func_nums + 1) {
            let mut _vec_hi = Vec::<Integer>::new();
            let mut _vec_temp = Vec::<Integer>::new();
            get_vec_hi(&self._vec_h, &_random_y, lens, &self._p, &self._q, &mut _vec_hi);
            get_vec_temp(&_random_y, &_random_z, &self._p, lens, &mut _vec_temp);
            let mut gz = Integer::from(1);
            for i in 0..lens {
                let a = self._vec_g[i].clone().pow_mod(&_random_z, &self._q).unwrap();
                let b = a.pow_mod(&Integer::from(-1), &self._q).unwrap();
                gz *= b;
            }
            gz %= self._q.clone();
            let mut hp = Integer::from(1);
            for i in 0..lens {
                let a = _vec_hi[i].clone().pow_mod(&_vec_temp[i], &self._q).unwrap();
                hp *= a;
            }
            hp %= self._q.clone();
            let temp5 = _s_up[iter].clone().pow_mod(&_random_x, &self._q).unwrap();
            let PL = _a_up[iter].clone() * temp5 * gz * hp % self._q.clone();
            let a = self._h.clone().pow_mod(&_mu[iter], &self._q).unwrap();
            let temp_p_up = a.pow_mod(&Integer::from(-1), &self._q).unwrap();
            self._p_up.push(temp_p_up * PL % self._q.clone());
            //content.push_str(&format!("_vec_hi {}", temps));
            if iter == 0 {
                let temps = _vec_hi.iter().map(|x| format!("{};", x)).collect::<String>();
                content += "_vec_h ";
                content += &temps;
                //println!("content : {}", content);
            }
            lens /= 2;
        }
        let mut alpha = Vec::<Integer>::new();
        for iter in 1..(self.func_nums + 1) {
            alpha.push(self.func_pk[iter].clone().pow_mod(&(gamma_vec[0].clone() - gamma_vec[iter].clone()), &self._q).unwrap());
        }
        //println!("_p_up: {:?}", self._p_up);
        content += "\n_p_up ";
        content += &(self._p_up.iter().map(|x| format!("{};", x)).collect::<String>());
        content += "\n_c ";
        content += &(_hat_t.iter().map(|x| format!("{};", x)).collect::<String>());
        content += "\nalpha ";
        content += &(alpha.iter().map(|x| format!("{};", x)).collect::<String>());
        content += "\n_v_up ";
        content += &(_v_up.iter().map(|x| format!("{};", x)).collect::<String>());
        write_to_file(pass_name, &content);
        for i in 0..self.bit_length {
            self._vec_h[i].assign( _vec_hi2[i].clone());
        }
        //println!("randomx y z : {};{};{}", _random_x, _random_y, _random_z);
        self.fnizk_synthesis(&_random_x, &_random_y, &_random_z, &_hat_t, &_tau_x, &gamma_vec, &_v_up, &_s_up, &_t1_up, &_t2_up, &t1_vec, &t2_vec, &tau1_vec, &tau2_vec);
        println!("FNIZK Verification PASS!!!");
    }
    fn fnizk_synthesis(&mut self, _random_x : &Integer, _random_y : &Integer, _random_z : &Integer, _hat_t : &Vec<Integer>, _tau_x : &Vec<Integer>, gamma_vec : &Vec<Integer>, _v_up : &Vec<Integer>, _s_up : &Vec<Integer>, _t1_up : &Vec<Integer>, _t2_up : &Vec<Integer>, _t1_vec : &Vec<Integer>, _t2_vec : &Vec<Integer>, _tau1_vec : &Vec<Integer>, _tau2_vec : &Vec<Integer>) {
        let mut lens = self.bit_length;
        let mut deltas = Vec::<Integer>::new();
        for _i in 0..(self.func_nums + 1) {
            let mut _delta = Integer::new();
            get_delta(_random_z, _random_y, lens, &self._p, &mut _delta) ;
            deltas.push(_delta);
            lens /= 2;
        }
        // test begin ...
        let _m = self.func_nums + 1;
        let mut _sum_t = Integer::from(1);
        for i in 0..self.func_nums {
            _sum_t += _hat_t[i+1].clone();
        }
        let _sum_ind = self.func_nums * _hat_t[0].clone() - _sum_t ;
        let _res_1 = self._g.clone().pow_mod(&_sum_ind, &self._q).unwrap();
        let mut _mul_dot = Integer::from(1);
        for i in 0..self.func_nums {
            _mul_dot *= self._g.clone().pow_mod(&(_hat_t[0].clone() - _hat_t[i+1].clone()), &self._q).unwrap();
        }
        let mut pks = Integer::from(1);
        for i in 0..self.func_nums {
            pks *= self.func_pk[i+1].clone();
        }
        pks %= self._q.clone();
        let _hl = (pks * self._h.clone().pow_mod(&Integer::from(self.func_nums), &self._q).unwrap() % self._q.clone()).pow_mod(&_tau_x[0], &self._q).unwrap();
        let mut _hr = Integer::from(1);
        for i in 0..self.func_nums {
            let _htemp = (self._h.clone()*self.func_pk[i+1].clone()).pow_mod(&_tau_x[i+1], &self._q).unwrap();
            _hr *= _htemp.pow_mod(&Integer::from(-1), &self._q).unwrap();
        }
        //println!("_hr*_hl: {}", _hr * _hl % self._q.clone());
        let mut _hres = Integer::from(1);
        for i in 0..self.func_nums {
            _hres *= (self._h.clone() * self.func_pk[i+1].clone()).pow_mod(&(_tau_x[0].clone() - _tau_x[i+1].clone()), &self._q).unwrap();
        }
        //println!("_hres: {}", _hres % self._q.clone());
        // test end ...
        // get check l
        let mut temp1 = Integer::new();
        for i in 0..self.func_nums {
            temp1 += _hat_t[i+1].clone();
        }
        temp1.assign(self.func_nums*_hat_t[0].clone() - temp1.clone());
        temp1.assign(self._g.clone().pow_mod(&temp1, &self._q).unwrap());
        let mut temp2 = Integer::from(1);
        for i in 0..self.func_nums {
            temp2 *= self.func_pk[i+1].clone();
        }
        temp2.assign((temp2.clone() * self._h.clone().pow_mod(&Integer::from(self.func_nums), &self._q).unwrap() % self._q.clone()).pow_mod(&_tau_x[0], &self._q).unwrap());
        let mut temp3 = Integer::from(1);
        for i in 0..self.func_nums {
            temp3 *= (self._h.clone() * self.func_pk[i+1].clone()).pow_mod(&(-1*_tau_x[i+1].clone()), &self._q).unwrap();
        }
        let check_l = temp1 * temp2 * temp3 % self._q.clone();
        //println!("check_l: {}" , check_l);
        // get check r ...
        temp1 = Integer::from(1);
        for i in 0..self.func_nums {
            temp1 *= self.func_pk[i+1].clone().pow_mod(&(gamma_vec[0].clone() - gamma_vec[i+1].clone()), &self._q).unwrap();
        }
        temp1 *= _v_up[0].clone().pow_mod(&Integer::from(self.func_nums), &self._q).unwrap() ;
        temp2 = Integer::from(1);
        for i in 0..self.func_nums {
            temp2 *= _v_up[i+1].clone().pow_mod(&Integer::from(-1), &self._q).unwrap();
        }
        temp1.assign((temp1.clone() * temp2).pow_mod(&(_random_z.clone()*_random_z.clone()%self._p.clone()),&self._q).unwrap());
        temp2 = Integer::from(0);
        for _i in 0..self.func_nums {
            temp2 += deltas[_i+1].clone();
        }
        temp2.assign(self.func_nums * deltas[0].clone() - temp2.clone());
        temp2.assign(self._g.clone().pow_mod(&temp2, &self._q).unwrap());
        temp3 = Integer::from(1);
        for i in 0..self.func_nums {
            temp3 *= _t1_up[i+1].clone();
        }
        temp3.assign(temp3.clone().pow_mod(&Integer::from(-1), &self._q).unwrap() * _t1_up[0].clone() % self._q.clone());
        temp3.assign(temp3.clone().pow_mod(&_random_x, &self._q).unwrap());
        let mut temp4 = Integer::from(1);
        for i in 0..self.func_nums {
            temp4 *= _t2_up[i+1].clone();
        }
        temp4.assign(temp4.clone().pow_mod(&Integer::from(-1), &self._q).unwrap() * _t2_up[0].clone() % self._q.clone());
        temp4.assign(temp4.clone().pow_mod(&(_random_x.clone()*_random_x.clone()), &self._q).unwrap());
        let check_r = temp1 * temp2 * temp3 * temp4 % self._q.clone();
        println!("check_l: {}" , check_l);
        println!("check_r: {}" , check_r);
        assert_eq!(check_l, check_r);
        /*
        //line 2 
        temp1 = Integer::from(1);
        for i in 0..self.func_nums {
            temp1 *= self._g.clone().pow_mod(&(_hat_t[0].clone() - _hat_t[i+1].clone()), &self._q).unwrap() * (self._h.clone() * self.func_pk[i+1].clone()).pow_mod(&(_tau_x[0].clone()-_tau_x[i+1].clone()), &self._q).unwrap();
        }
        println!("line 2: {}", temp1 % self._q.clone());
        //line 3 
        let mut ln = Integer::from(1);
        let mut temp_l3 = Integer::from(1);
        for i in 0..self.func_nums {
            let alphas = self.func_pk[i+1].clone().pow_mod(&(gamma_vec[0].clone() - gamma_vec[i+1].clone()), &self._q).unwrap();
            let vi = _v_up[i+1].clone().pow_mod(&Integer::from(-1), &self._q).unwrap();
            let l1 = (alphas * _v_up[0].clone() * vi).pow_mod(&(_random_z.clone()*_random_z.clone()), &self._q).unwrap();
            let l2 = self._g.clone().pow_mod(&(deltas[0].clone() - deltas[i+1].clone()), &self._q).unwrap();
            let gs1 = self._g.clone().pow_mod(&t1_vec[0], &self._q).unwrap();
            let hs1 = (self._h.clone() * self.func_pk[i+1].clone()).pow_mod(&tau1_vec[0], &self._q).unwrap();
            let ts1 = t1_vec[i+1].clone().pow_mod(&Integer::from(-1), &self._q).unwrap();
            let l3 = (gs1 * hs1 * ts1).pow_mod(&_random_x, &self._q).unwrap();
            temp_l3 *= l3.clone();
            let gs2 = self._g.clone().pow_mod(&t2_vec[0], &self._q).unwrap();
            let hs2 = (self._h.clone() * self.func_pk[i+1].clone()).pow_mod(&tau2_vec[0], &self._q).unwrap();
            let ts2 = t2_vec[i+1].clone().pow_mod(&Integer::from(-1), &self._q).unwrap();
            let l4 = (gs2*hs2*ts2).clone().pow_mod(&(_random_x.clone() * _random_x.clone()), &self._q).unwrap();
            ln = ln * l1 * l2 * l3 * l4 % self._q.clone();
        }
        println!("line 3: {}" , ln);
        //println!("temp_l3: {}", temp_l3%self._q.clone());
        // line 4, again
        let mut alphas = Integer::from(1);
        for i in 0..self.func_nums {
            alphas *= self.func_pk[i+1].clone().pow_mod(&(gamma_vec[0].clone()-gamma_vec[i+1].clone()), &self._q).unwrap();
        }
        let v0 = _v_up[0].clone().pow_mod(&Integer::from(self.func_nums), &self._q).unwrap();
        let mut vs = Integer::from(1);
        let mut delta_temp = Integer::new();
        for i in 0..self.func_nums {
            vs *= _v_up[i+1].clone().pow_mod(&Integer::from(-1), &self._q).unwrap();
            delta_temp += deltas[i+1].clone();
        }
        let k1 = (alphas * v0 * vs).pow_mod(&(_random_z.clone()*_random_z.clone()), &self._q).unwrap();
        println!("k1: {}", k1);
        let k2 = self._g.clone().pow_mod(&(self.func_nums*deltas[0].clone()-delta_temp), &self._q).unwrap();
        println!("k2: {}", k2);
        let mut ts = Integer::from(1);
        for i in 0..self.func_nums {
            ts *= _t1_up[i+1].clone();
        }
        let k3 = (ts.pow_mod(&Integer::from(-1), &self._q).unwrap()*_t1_up[0].clone()).pow_mod(&_random_x, &self._q).unwrap();
        println!("k3: {}", k3);
        let mut ts2 = Integer::from(1);
        for i in 0..self.func_nums {
            ts2 *= _t2_up[i+1].clone();
        }
        let k4 = (ts2.pow_mod(&Integer::from(-1), &self._q).unwrap() * _t2_up[0].clone()).pow_mod(&(_random_x.clone()*_random_x.clone()), &self._q).unwrap();
        println!("k4: {}", k4);
        let ks = k1 * k2 * k3 * k4 % self._q.clone();
        println!("new line 4: {}", ks); */
    }
}


