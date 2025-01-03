import gmpy2
from gmpy2 import mpz
import time
import hashlib

from utils import *

class Prover():
    
    def __init__(self, ):
        # public parameters
        self._p = 0
        self._q = 0
        self._h = 0
        self._g = 0
        self._u = 0
        self._vec_g = []
        self._vec_h = []
        self._P = 0
        self._c = 0

        # private parameters
        self._V = self._A = self._S = 0
        self._tau_x = self._mu = self._hat_t = 0
        self._T1 = self._T2 = 0
        
        self.bit_len = 0
        self.random_state = None
    
    def inner_product_argument(self, ):
        _random_xx_data = get_msg_phug(self._P, self._u, self._vec_g, self._vec_h)
        _random_xx = int(hashlib.sha256(_random_xx_data.encode('utf-8')).hexdigest(), 16) % self._p
        
        self._P = pow(self._u, _random_xx * self._c, self._q) * self._P % self._q
        self._u = pow(self._u, _random_xx, self._q)
        print("prover self._P: " + str(self._P))
        print("prover self._u: " + str(self._u))
        print("prover self._p: " + str(self._p))
        self._array_l = [] #* self.bit_len 
        self._array_r = [] #* self.bit_len 
        self.inner_product_argument_proof(self.bit_len )
        
    def inner_product_argument_proof(self, _bit_len ):
        if (_bit_len == 1):
            print("prover self-verifier")
            temp1 = commit_s(self._vec_g[0], self._vec_a[0], self._vec_h[0], self._vec_b[0], self._q)
            tempc = pow(self._u, self._vec_a[0] * self._vec_b[0]%self._p, self._q) * temp1 % self._q
            print("prover _P: " + str(tempc))
            print(self._u)
            print(self._vec_a[0])
            print(self._vec_b[0])
            print(self._vec_g[0])
            print(self._vec_h[0])
            print(self._p)
            print(self._q)

            return;
        _bit_len = _bit_len // 2
        c_L = inner_product(self._vec_a, 0, self._vec_b, _bit_len, _bit_len, self._p)
        c_R = inner_product(self._vec_a, _bit_len, self._vec_b, 0, _bit_len, self._p)
        L = commit_v(self._vec_g[_bit_len:], self._vec_a[:_bit_len], self._vec_h[:_bit_len], self._vec_b[_bit_len:], _bit_len, self._q) * pow(self._u, c_L, self._q) % self._q
        R = commit_v(self._vec_g[:_bit_len], self._vec_a[_bit_len:], self._vec_h[_bit_len:], self._vec_b[:_bit_len], _bit_len, self._q) * pow(self._u, c_R, self._q) % self._q
        self._array_l.append(L)
        self._array_r.append(R)
        _random_xx = int(hashlib.sha256((hex(L)[2:]+hex(R)[2:]).encode('utf-8')).hexdigest(), 16) % self._p
        xx_inv = pow(_random_xx, -1, self._p)
        _temp_gl = [pow(self._vec_g[j], xx_inv, self._q) for j in range(_bit_len)]
        _temp_gr = [pow(self._vec_g[j+_bit_len], _random_xx, self._q) for j in range(_bit_len)]
        self._vec_g = [(_temp_gl[j] * _temp_gr[j])%self._q for j in range(_bit_len)]
        _temp_hl = [pow(self._vec_h[j], _random_xx, self._q) for j in range(_bit_len)]
        _temp_hr = [pow(self._vec_h[j+_bit_len], xx_inv, self._q) for j in range(_bit_len)]
        self._vec_h = [(_temp_hl[j] * _temp_hr[j])%self._q for j in range(_bit_len)]
        xx_sqr = _random_xx * _random_xx % self._p
        xx_sqr_inv = pow(xx_sqr, -1, self._p)
        self._P = pow(L, xx_sqr, self._q) * self._P * pow(R, xx_sqr_inv, self._q) % self._q
        print("prover, self._P: " + str(self._P))
        self._vec_a = [(self._vec_a[j] * _random_xx + self._vec_a[j+_bit_len] * xx_inv) % self._p for j in range(_bit_len)]
        self._vec_b = [(self._vec_b[j] * xx_inv + self._vec_b[j+_bit_len] * _random_xx) % self._p for j in range(_bit_len)]
        
        return self.inner_product_argument_proof( _bit_len )
        

    def range_proof(self, witness, is_output):
        self._vec_a = None #[0] * self.bit_len
        self._vec_b = None #[0] * self.bit_len 
        al = [0] * self.bit_len
        ar = [0] * self.bit_len

        SL = [0] * self.bit_len 
        SR = [0] * self.bit_len 
        for i in range(self.bit_len):
            SL[i] = int(gmpy2.mpz_random(self.random_state, self._p))
            SR[i] = int(gmpy2.mpz_random(self.random_state, self._p))
        _alpha = _rho = _gamma = 0
        _random_x = _random_y = _random_z = 0
        _tau1 = _tau2 = 0
        _t1 = _t2 = 0
        lens = self.bit_len
        _witness = gmpy2.digits(mpz(witness), 2)
        _witness = _witness[::-1]
        _witness += "0"* (self.bit_len - len(_witness))
        for  i in range(self.bit_len):
            al[i] = int(_witness[i])
            ar[i] = (al[i] - 1 ) % self._p
        _gamma = int(gmpy2.mpz_random(self.random_state, self._p))
        _alpha = int(gmpy2.mpz_random(self.random_state, self._p))
        _rho = int(gmpy2.mpz_random(self.random_state, self._p))
        _tau1 = int(gmpy2.mpz_random(self.random_state, self._p))
        _tau2 = int(gmpy2.mpz_random(self.random_state, self._p))
        self._V = commit_s(self._g, witness, self._h, _gamma, self._q)

        self._A = (pow(self._h, _alpha, self._q) * commit_v(self._vec_g, al, self._vec_h, ar, lens, self._q)) % self._q
    
        self._S = (pow(self._h, _rho, self._q) * commit_v(self._vec_g, SL, self._vec_h, SR, lens, self._q)) % self._q
        _random_y = int(hashlib.sha256((hex(self._A)[2:] + hex(self._S)[2:]).encode('utf-8')).hexdigest(), 16) % self._p
        _random_z = int(hashlib.sha256((hex(self._A)[2:] + hex(self._S)[2:] + hex(_random_y)[2:]).encode('utf-8')).hexdigest(), 16) % self._p

        _t1 = get_t1(al, ar, SL, SR, _random_y, _random_z, lens, self._p)
        _t2 = get_t2(SL, SR, _random_y, lens, self._p)

        self._T1 = commit_s(self._g, _t1, self._h, _tau1, self._q)
        self._T2 = commit_s(self._g, _t2, self._h, _tau2, self._q)
        _random_x = int(hashlib.sha256((hex(self._T1)[2:]+hex(self._T2)[2:]).encode('utf-8')).hexdigest(), 16) % self._p
        self._vec_a = get_lx(al, SL, _random_x, _random_z, lens, self._p)
        self._vec_b = get_rx(ar, SR, _random_x, _random_y, _random_z, lens, self._p)
    
        self._hat_t = inner_product(self._vec_a, 0, self._vec_b, 0, lens, self._p)

        self._tau_x = (_tau2 * _random_x * _random_x + _tau1 * _random_x + _random_z * _random_z * _gamma ) % self._p
        self._mu = (_alpha + _rho * _random_x) % self._p

        _vec_hi = get_vec_hi(self._vec_h, _random_y, lens, self._p, self._q)
        print("prover _vec_hi")
        print(_vec_hi)
        p_prime = pow(self._h, self._mu, self._q) * commit_v(self._vec_g, self._vec_a, _vec_hi, self._vec_b, lens, self._q) % self._q
        print("p prime :" + str(p_prime))
        if is_output :
            print("_witness hex: "  + str(witness))
            print("_witness bit: "  + _witness )
            print(al)
            print(ar)
            _t1 = _t2 = 0
            for i in range(self.bit_len):
                _t1 += al[i] * pow(2, i)
                _t2 += al[i] * ar[i]
            print("al * z^n : " + str(_t1))
            print("al * ar: " + str(_t2))

            print("SL")
            print(SL)
            print("SR")
            print(SR)
            print("_rho: " + str(_rho))
            print("_alpha: " + str(_alpha))
            print("_gamma: " +str(_gamma))
            print("_random_x: " +str(_random_x))
            print("_random_y: " +str(_random_y))
            print("_random_z: " +str(_random_z))
            print("_tau1: " + str(_tau1))
            print("_tau2: " + str(_tau2))
            print("_tau_x: " + str(self._tau_x))
            print("_hat_t: " + str(self._hat_t))
            print("_mu: " +str(self._mu))
            print("vector a: ")
            print(self._vec_a)
            print("vector b: ")
            print(self._vec_b)
            print("_t1: " + str(_t1))
            print("_t2: " + str(_t2))
            print("_T1: " + str(self._T1))
            print("_T2: " + str(self._T2))
            print("_V: " + str(self._V))
            print("_A: " + str(self._A))
            print("_S: " + str(self._S))
            _vec_hi_test = get_vec_hi(self._vec_h, _random_y, lens, self._p, self._q)
            PR_mu = commit_v(self._vec_g, self._vec_a, _vec_hi_test, self._vec_b, lens, self._q) * pow(self._h, self._mu, self._q) % self._q
            print("PR     : " + str(PR_mu))


