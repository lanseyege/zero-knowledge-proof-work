import gmpy2
from gmpy2 import mpz
import time
import hashlib
import math

from utils import *


class Verifier():
    
    def __init__(self, ):
        self._p = 0
        self._q = 0
        self._h = 0
        self._g = 0
        self._u = 0
        self._vec_g = []
        self._vec_h = []
        self._P = 0
        self._c = 0
        
        self.bit_len = 0
        self.random_state = None

        self._random_x = 0
        self._random_y = 0
        self._random_z = 0

    def verifier_range_proof(self, _tau_x, _mu, _hat_t, _V, _A, _S, _T1, _T2, is_output):
        lens = self.bit_len
        #_random_x = int(hashlib.sha256((hex(_T1)[2:]+hex(_T2)[2:]).encode('utf-8')).hexdigest(), 16) % self._p
        #_random_y = int(hashlib.sha256((hex(_A)[2:] + hex(_S)[2:]).encode('utf-8')).hexdigest(), 16) % self._p
        #_random_z = int(hashlib.sha256((hex(_A)[2:] + hex(_S)[2:] + hex(_random_y)[2:]).encode('utf-8')).hexdigest(), 16) % self._p
        # check tx 
        #check_tl = commit_s(self._g, _hat_t, self._h, _tau_x, self._q)

        self.delta = get_delta(self._random_z, self._random_y, lens, self._p)
        #check_tr = (pow(_V, self._random_z * self._random_z, self._q) * pow(self._g, delta, self._q) %self._q) * (pow(_T1, self._random_x, self._q) * pow(_T2, self._random_x * self._random_x, self._q) %self._q) % self._q
        
        #print("check_tl: " + str(check_tl))
        #print("check_tr: " + str(check_tr))

        #assert(check_tl == check_tr)

        _vec_hi = get_vec_hi(self._vec_h, self._random_y, lens, self._p, self._q)
        vec_temp = get_vec_temp(self._random_y, self._random_z, lens, self._p)
        gz = 1
        for i in range(lens):
            a = pow(self._vec_g[i], self._random_z, self._q)
            b = pow(a, -1, self._q)
            gz *= b
        gz %= self._q
        hp = 1
        for i in range(lens):
            a = pow(_vec_hi[i], vec_temp[i], self._q)
            hp *= a
        hp %= self._q
        PL = _A * pow(_S, self._random_x, self._q) * gz * hp % self._q
        a = pow(self._h, _mu, self._q)
        self._P = pow(a, -1, self._q) * PL % self._q
        self._vec_h = _vec_hi[:]
        if is_output:
            print("_random_x: " +str(self._random_x))
            print("_random_y: " +str(self._random_y))
            print("_random_z: " +str(self._random_z))
            print("PL ---: " + str(PL))
            print("new _P: " + str(self._P))
            print("_vec_hi: ")
            print(_vec_hi)
            print("vec_temp: ")
            print(vec_temp)

        #return check_tl == check_tr
        return 1

    def verifier_inner_product_argument(self, _vec_g0, _vec_h0, _vec_a0, _vec_b0, _array_l, _array_r):
        _bit_len = self.bit_len
        _random_xx_data = get_msg_phug(self._P, self._u, self._vec_g, self._vec_h)
        _random_xx = int(hashlib.sha256(_random_xx_data.encode('utf-8')).hexdigest(), 16) % self._p
        
        self._P = pow(self._u, _random_xx * self._c, self._q) * self._P % self._q
        self._u = pow(self._u, _random_xx, self._q)
        print("verifier self._P: " + str(self._P))
        print("verifier self._u: " + str(self._u))
        print("verifier self._p: " + str(self._p))
        _logn = math.ceil(math.log(self.bit_len, 2))
        for i in range(_logn):
            _bit_len = _bit_len // 2
            print("verifier _bit_len: " + str(_bit_len))
            _random_xx = int(hashlib.sha256((hex(_array_l[i])[2:] + hex(_array_r[i])[2:]).encode('utf-8')).hexdigest(), 16) % self._p
            print("verifier _random_x: " + str(_random_xx))
            xx_inv = pow(_random_xx, -1, self._p)
            _temp_gl = [pow(self._vec_g[j], xx_inv, self._q) for j in range(_bit_len)]
            _temp_gr = [pow(self._vec_g[j+_bit_len], _random_xx, self._q) for j in range(_bit_len)]
            self._vec_g = [(_temp_gl[j] * _temp_gr[j]) % self._q for j in range(_bit_len)]
            _temp_hl = [pow(self._vec_h[j], _random_xx, self._q) for j in range(_bit_len)]
            _temp_hr = [pow(self._vec_h[j+_bit_len], xx_inv, self._q) for j in range(_bit_len)]
            self._vec_h = [(_temp_hl[j] * _temp_hr[j])%self._q for j in range(_bit_len)]
            xx_sqr = _random_xx * _random_xx
            xx_sqr_inv = pow(xx_sqr, -1, self._p)
            self._P = pow(_array_l[i], xx_sqr, self._q) * self._P * pow(_array_r[i], xx_sqr_inv, self._q) % self._q
            print("verifier, self._P: " + str(self._P))
            print("verifer g h")
            print(self._vec_g)
            print(self._vec_h)
        print("verifier g0: " + str(self._vec_g[0]))
        print("verifier h0: " + str(self._vec_h[0]))
        P_prime = pow(self._u, _vec_a0 * _vec_b0 % self._p, self._q) * commit_s(self._vec_g[0], _vec_a0, self._vec_h[0], _vec_b0, self._q) % self._q
        print("c = a * b: " + str(_vec_a0 * _vec_b0 % self._p) )

        print("self._P: " + str(self._P))
        print("P_prime: " + str(P_prime))
        print(self._u)
        print(_vec_a0)
        print(_vec_b0)
        print(self._vec_g[0])
        print(self._vec_h[0])
        print(self._p)
        print(self._q)
        assert(self._P, P_prime)
        return self._P == P_prime
        


