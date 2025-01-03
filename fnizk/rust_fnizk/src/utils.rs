#![allow(non_snake_case)]
use std::fs::File;
use std::io::prelude::*;
use toml::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use rug::{Assign, Integer};
use rug::rand::RandState;
//use sha256::digest;
    
pub fn write_to_file(file_name : &str , content : &str) {
    let mut file = File::create(file_name).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

pub fn read_from_file(file_name : &str, content : &mut String) {
    let mut file = File::open(file_name).unwrap();
    //let mut contents = String::new();
    file.read_to_string(content).unwrap();
    //contents
}

pub fn get_config(file_name : &str) -> (usize, u32, u32, u32, u32, u32, usize) {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let value = contents.parse::<Value>().unwrap();
    let bit_length = value["params"]["bit_length"].as_integer().unwrap() as usize;
    let p_length = value["params"]["p_length"].as_integer().unwrap() as u32;
    let witness = value["params"]["witness"].as_integer().unwrap() as u32;
    let prime_reps = value["params"]["prime_reps"].as_integer().unwrap() as u32;
    let is_random = value["params"]["is_random"].as_integer().unwrap() as u32;
    let rand_seed = value["params"]["rand_seed"].as_integer().unwrap() as u32;
    let func_nums = value["params"]["func_nums"].as_integer().unwrap() as usize;
    (bit_length, p_length, witness, prime_reps, is_random, rand_seed, func_nums)
}

pub fn get_sys_time_in_secs() -> u128 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis(),
        Err(_) => panic!("SystemTime before unix epoch!"),
    }
}

pub fn get_pugh(_p_up : &Integer, _u : &Integer, _vec_g : &Vec<Integer>, _vec_h : &Vec<Integer>, content : &mut String) {
    content.push_str(&format!("{}", _p_up.clone()));
    content.push_str(&format!("{}", _u.clone()));
    for temp in _vec_g.clone().iter() {
        content.push_str(&format!("{}", temp));
    }
    for temp in _vec_h.clone().iter() {
        content.push_str(&format!("{}", temp));
    }
}
/*
pub fn inner_product(left : &Vec<Integer>, start1 : usize, right : &Vec<Integer> , start2 : usize, lens : usize, modulo : &Integer, res : &mut Integer) {
    for i in 0..lens {
        *res += left[i + start1].clone() * right[i + start2].clone();
    }
    //res.assign(res.pow_mod(one, modulo).unwrap());
    *res %= (*modulo).clone();
}*/

pub fn generator_yield(_prime : &mut Integer, rand : &mut RandState, p_length : u32, _p : &mut Integer, _q : &mut Integer) {
    _prime.assign(Integer::from(Integer::random_bits(p_length, rand)).next_prime()); 
    //let mut temp1 = 0;
    //let mut temp2 = 0;
    let two = Integer::from(2);
    loop {
        let temp1 = match _prime.clone().pow_mod(_p, _q){
            Ok(temp1) => temp1,
            Err(_) => unreachable!(),
        };
        let temp2 = match _prime.clone().pow_mod(&two, _q) {
            Ok(temp2) => temp2,
            Err(_) => unreachable!(),
        };
        if temp1 == 1 && temp2 != 1 && _prime != _p {
            break ;
        } else {
            _prime.assign(Integer::from(Integer::random_bits(p_length, rand)).next_prime()); 
        }
    }   
}

pub fn read_variables(file_name : &str, _p : &mut Integer, _q : &mut Integer, _g : &mut Integer, _h : &mut Integer, _u : &mut Integer, _vec_g : &mut Vec<Integer>, _vec_h : &mut Vec<Integer>, func_pk : &mut Vec<Integer>) {
    let mut content = String::new();
    read_from_file(file_name , &mut content);
    //println!("content: {}" , content);
    //println!("digest: {}", digest(content.clone()));
    let result = content.split('\n');
    for res in result {
        if !res.is_empty() {
            //let mut temp = res.next().unwrap();
            let mut res2 = res.split(' ');
            let f1 = res2.next().unwrap();
            let f2 = res2.next().unwrap();
            //println!("f1 {}", f1.clone());
            //println!("f2 {}", f2.clone());
            match f1 {
                "p" => _p.assign(Integer::parse_radix(f2, 10).unwrap()), 
                "q" => _q.assign(Integer::parse_radix(f2, 10).unwrap()), 
                "g" => _g.assign(Integer::parse_radix(f2, 10).unwrap()), 
                "h" => _h.assign(Integer::parse_radix(f2, 10).unwrap()), 
                "u" => _u.assign(Integer::parse_radix(f2, 10).unwrap()), 
                "vec_g" => {
                    if _vec_g.len() != 0 {
                        _vec_g.drain(..);
                    }
                    let f3 = f2.split(';');
                    //while f3.remainder() != None {
                    for _f3 in f3 {
                        if ! _f3.is_empty() {
                            _vec_g.push(Integer::from_str_radix(_f3, 10).unwrap());
                        }
                    }
                },
                "vec_h" => {
                    if _vec_h.len() != 0 {
                        _vec_h.drain(..);
                    }
                    let f3 = f2.split(';');
                    //while f3.remainder() != None {
                    for _f3 in f3 {
                        if ! _f3.is_empty() {
                            _vec_h.push(Integer::from_str_radix(_f3, 10).unwrap());
                        }
                    }
                },
                "func_pk" => {
                    if func_pk.len() != 0 {
                        func_pk.drain(..);
                    }
                    let f3 = f2.split(';');
                    for _f3 in f3 {
                        if ! _f3.is_empty() {
                            func_pk.push(Integer::from_str_radix(_f3, 10).unwrap());
                        }
                    }
                },
                _ => println!("something wrong!!"),
            }
        }
    }
}


pub fn commit_s(base1: Integer, inx1 : &Integer, base2 : Integer, inx2 : &Integer, modulo : Integer, res : &mut Integer ) {
    let temp1 = match base1.pow_mod(inx1, &modulo) {
        Ok(temp1) => temp1,
        Err(_) => unreachable!(),
    };
    let temp2 = match base2.pow_mod(inx2, &modulo) {
        Ok(temp2) => temp2,
        Err(_) => unreachable!(),
    };
    res.assign(temp1 * temp2 % modulo);
}

pub fn commit_v(vec1 : &Vec<Integer>, inx1 : &Vec<Integer>, vec2 : &Vec<Integer>, inx2 : &Vec<Integer>, lens : usize, modulo : Integer, res : &mut Integer) {
    //res.assign(1);
    let mut tempk = Vec::<Integer>::new();
    for i in 0..lens {
        let temp1 = match vec1[i].clone().pow_mod(&inx1[i], &modulo) {
            Ok(temp1) => temp1,
            Err(_) => unreachable!(),
        };
        let temp2 = match vec2[i].clone().pow_mod(&inx2[i], &modulo) {
            Ok(temp2) => temp2,
            Err(_) => unreachable!(),
        };
        tempk.push( temp1);
        tempk.push( temp2);
    }
    res.assign(tempk.iter().product::<Integer>() % modulo);
}

pub fn get_t1(al : &Vec<Integer>, ar : &Vec<Integer>, SL : &Vec<Integer>, SR : &Vec<Integer>, y : &Integer, z : &Integer, lens : usize, modulo : &Integer, res : &mut Integer) {
    let mut _y = Integer::from(1);
    let mut _tw = Integer::from(1);
    let mut temp = Integer::new();
    for i in 0..lens {
        temp = temp + ((al[i].clone() - z.clone()) * (_y.clone() * SR[i].clone()) + SL[i].clone() * (_y.clone() * (ar[i].clone() + z.clone()) + z.clone() * z.clone() * _tw.clone())) % modulo.clone() ;
        _y = (_y * y.clone()) % modulo.clone();
        _tw = _tw * 2;
    }
    res.assign(temp % modulo.clone());
}

pub fn get_t2(SL : &Vec<Integer>, SR : &Vec<Integer>, y : &Integer, lens : usize, modulo : &Integer, res : &mut Integer) {
    let mut _y = Integer::from(1);
    let mut temp = Integer::new();
    for i in 0..lens {
        temp = temp + SL[i].clone() * _y.clone() * SR[i].clone() % modulo.clone();
        _y = _y * y.clone() % modulo.clone();
    }
    res.assign(temp % modulo.clone());
}

pub fn get_lx(al : &Vec<Integer>, SL : &Vec<Integer>, x : &Integer, z : &Integer, lens : usize, modulo : &Integer, res : &mut Vec<Integer>) {
    for i in 0..lens {
        res.push((al[i].clone() - z.clone() + SL[i].clone() * x.clone()) % modulo.clone());
    }
}

pub fn get_rx(ar : &Vec<Integer>, SR : &Vec<Integer>, x : &Integer, y : &Integer, z : &Integer, lens : usize, modulo : &Integer, res : &mut Vec<Integer>) {
    let mut _y = Integer::from(1);
    let mut _two = Integer::from(1);
    for i in 0..lens {
        //res.push((_y.clone() * (ar[i].clone() + z.clone() + SR[i].clone() * x.clone())%modulo.clone() + (z.clone() * z.clone() % modulo.clone()) * _two.clone()) % modulo.clone()) ;
        res.push((_y.clone() * (ar[i].clone() + z.clone() + SR[i].clone() * x.clone()) + z.clone() * z.clone() * _two.clone()) % modulo.clone()) ;
        _y = _y * y.clone() % modulo.clone();
        _two = _two * 2;
    }
}

pub fn inner_product(left : &Vec<Integer>, start1 : usize, right : &Vec<Integer>, start2 : usize, lens : usize, modulo : &Integer, res : &mut Integer) {
    let mut temp = Integer::new();
    for i in 0..lens {
        temp = temp + left[i+start1].clone() * right[i+start2].clone() ;
    }
    res.assign(temp % modulo.clone());
}

pub fn get_delta(z : &Integer, y : &Integer, lens : usize, modulo : &Integer, res : &mut Integer ) {
    let z2 = z.clone() * z.clone() ;
    let z3 = z2.clone() * z.clone();
    let mut _y = Integer::from(1);
    let mut _tw = Integer::from(1);
    let mut res1 = Integer::new();
    let mut res2 = Integer::new();
    for _i in 0..lens {
        res1 += _y.clone();
        res2 += _tw.clone();
        _y *= y;
        _tw *= 2;
    }
    res.assign( ((z.clone() - z2)*res1 - z3 * res2  ) % modulo.clone());
}

pub fn get_vec_hi(_vec_h : &Vec<Integer>, y : &Integer, lens : usize, p : &Integer, q : &Integer, _vec_hi : &mut Vec<Integer>) {
    _vec_hi.push(_vec_h[0].clone());
    for i in 1..lens {
        let c = match y.clone().pow_mod(&Integer::from(i), p) {
            Ok(c) => c,
            Err(_) => unreachable!(),
        };
        let a = match c.pow_mod(&Integer::from(-1), p) {
            Ok(a) => a,
            Err(_) => unreachable!(),
        };
        let b = match _vec_h[i].clone().pow_mod(&a, q) {
            Ok(b) => b,
            Err(_) => unreachable!(),
        };
        _vec_hi.push(b);
    }
}

pub fn get_vec_temp(y : &Integer, z : &Integer, modulo : &Integer, lens : usize, _vec_temp : &mut Vec<Integer>) {
    let mut _y = Integer::from(1);
    let mut _tw = Integer::from(1);
    for _i in 0..lens {
        _vec_temp.push((z.clone() * _y.clone() + z.clone() * z.clone() * _tw.clone()) % modulo.clone());
        _y *= y.clone();
        _tw *= 2;
    }
}


