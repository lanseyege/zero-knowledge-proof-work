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

        self.is_output = 1
    
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
        print("bit len: " + str(_bit_len))
        print("vec a b")
        print(self._vec_a)
        print(self._vec_b)
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

        print("c_L: " + str(c_L))
        print("c_R: " + str(c_R))
        print("L: " + str(L))
        print("R: " + str(R))
        print("_random_xx: " + str(_random_xx))
        print("_vec_g")
        print(self._vec_g)
        print("_vec_h")
        print(self._vec_h)
        print("P prime: " + str(self._P))
        print("vec a")
        print(self._vec_a)
        print("vec b")
        print(self._vec_b)
        
        return self.inner_product_argument_proof( _bit_len )
        

    def range_proof_before_yz(self, witness, is_output, _func_pk):
        self.is_output = is_output
        self._vec_a = None #[0] * self.bit_len
        self._vec_b = None #[0] * self.bit_len 
        self.al = [0] * self.bit_len
        self.ar = [0] * self.bit_len

        self.SL = [0] * self.bit_len 
        self.SR = [0] * self.bit_len 
        for i in range(self.bit_len):
            self.SL[i] = int(gmpy2.mpz_random(self.random_state, self._p))
            self.SR[i] = int(gmpy2.mpz_random(self.random_state, self._p))
        self._alpha = self._rho = self._gamma = 0
        self._random_x = self._random_y = self._random_z = 0
        self._tau1 = self._tau2 = 0
        self._t1 = self._t2 = 0
        lens = self.bit_len
        print("before yz, lens: " + str(lens))
        #print("before yz, lens: " + str(self.bit_len))
        self.witness = witness
        self._witness = gmpy2.digits(mpz(witness), 2)
        self._witness = self._witness[::-1]
        self._witness += "0"* (self.bit_len - len(self._witness))
        for  i in range(self.bit_len):
            self.al[i] = int(self._witness[i])
            self.ar[i] = (self.al[i] - 1 ) % self._p
        print("al: ")
        print(self.al)
        print("ar: ")
        print(self.ar)
        self._gamma = int(gmpy2.mpz_random(self.random_state, self._p))
        self._alpha = int(gmpy2.mpz_random(self.random_state, self._p))
        self._rho = int(gmpy2.mpz_random(self.random_state, self._p))
        self._tau1 = int(gmpy2.mpz_random(self.random_state, self._p))
        self._tau2 = int(gmpy2.mpz_random(self.random_state, self._p))
        self._V = commit_s(self._g, self.witness, self._h, self._gamma, self._q)

        self._A = (pow(self._h, self._alpha, self._q) * commit_v(self._vec_g, self.al, self._vec_h, self.ar, lens, self._q)) % self._q
    
        self._S = (pow(self._h, self._rho, self._q) * commit_v(self._vec_g, self.SL, self._vec_h, self.SR, lens, self._q)) % self._q
        return

    def range_proof_after_yz(self, _func_pk, _m, inx):
        #_random_y = int(hashlib.sha256((hex(self._A)[2:] + hex(self._S)[2:]).encode('utf-8')).hexdigest(), 16) % self._p
        #_random_z = int(hashlib.sha256((hex(self._A)[2:] + hex(self._S)[2:] + hex(_random_y)[2:]).encode('utf-8')).hexdigest(), 16) % self._p
        lens = self.bit_len
        print("after yz, lens: " + str(lens))
        self._t1 = get_t1(self.al, self.ar, self.SL, self.SR, self._random_y, self._random_z, lens, self._p)
        self._t2 = get_t2(self.SL, self.SR, self._random_y, lens, self._p)
        if inx == 0:
            _temp_h_pk = pow(self._h, _m, self._q) * _func_pk #% self._q
            self._T1 = commit_s(self._g, _m * self._t1, _temp_h_pk, self._tau1, self._q)
            self._T2 = commit_s(self._g, _m * self._t2, _temp_h_pk, self._tau2, self._q)    
        else:
            _temp_h_pk = self._h * _func_pk #% self._q
            self._T1 = commit_s(self._g, self._t1, _temp_h_pk, self._tau1, self._q)
            self._T2 = commit_s(self._g, self._t2, _temp_h_pk, self._tau2, self._q)    

        return

    def range_proof_after_T(self, ):
        #_random_x = int(hashlib.sha256((hex(self._T1)[2:]+hex(self._T2)[2:]).encode('utf-8')).hexdigest(), 16) % self._p
        lens = self.bit_len
        print("after T, lens: " + str(lens))
        self._vec_a = get_lx(self.al, self.SL, self._random_x, self._random_z, lens, self._p)
        self._vec_b = get_rx(self.ar, self.SR, self._random_x, self._random_y, self._random_z, lens, self._p)
    
        self._hat_t = inner_product(self._vec_a, 0, self._vec_b, 0, lens, self._p)

        self._tau_x = (self._tau2 * self._random_x * self._random_x + self._tau1 * self._random_x + self._random_z * self._random_z * self._gamma ) % self._p
        self._mu = (self._alpha + self._rho * self._random_x) % self._p

        if self.is_output :
            print("_witness hex: "  + str(self.witness))
            print("_witness bit: "  + self._witness )
            print(self.al)
            print(self.ar)
            t1_q = t2_q = 0
            for i in range(self.bit_len):
                t1_q += self.al[i] * pow(2, i)
                t2_q += self.al[i] * self.ar[i]
            print("al * z^n : " + str(t1_q))
            print("al * ar: " + str(t2_q))

            print("SL")
            print(self.SL)
            print("SR")
            print(self.SR)
            print("_rho: " + str(self._rho))
            print("_alpha: " + str(self._alpha))
            print("_gamma: " +str(self._gamma))
            print("_random_x: " +str(self._random_x))
            print("_random_y: " +str(self._random_y))
            print("_random_z: " +str(self._random_z))
            print("_tau1: " + str(self._tau1))
            print("_tau2: " + str(self._tau2))
            print("_tau_x: " + str(self._tau_x))
            print("_hat_t: " + str(self._hat_t))
            print("_mu: " +str(self._mu))
            print("vector a: ")
            print(self._vec_a)
            print("vector b: ")
            print(self._vec_b)
            print("_t1: " + str(self._t1))
            print("_t2: " + str(self._t2))
            print("_T1: " + str(self._T1))
            print("_T2: " + str(self._T2))
            print("_V: " + str(self._V))
            print("_A: " + str(self._A))
            print("_S: " + str(self._S))
            _vec_hi_test = get_vec_hi(self._vec_h, self._random_y, lens, self._p, self._q)
            PR_mu = commit_v(self._vec_g, self._vec_a, _vec_hi_test, self._vec_b, lens, self._q) * pow(self._h, self._mu, self._q) % self._q
            print("PR     : " + str(PR_mu))


