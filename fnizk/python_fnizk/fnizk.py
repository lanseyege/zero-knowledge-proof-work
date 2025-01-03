import gmpy2
from gmpy2 import mpz
import time
import hashlib

from utils import *
from prover import Prover
from verifier import Verifier

class FNIZK():
    def __init__(self, p_length, witness, bit_len, func_nums, seed, is_output):
        self.p_length = p_length
        self.witness = witness
        self.bit_len = bit_len
        self._p = mpz()
        self._q = mpz()
        self._g = 0
        self._h = 0
        self._u = 0
        self._vec_g = []
        self._vec_h = []
        self.random_state = None
        self.prime_test_reps = 10
        self.func_nums = func_nums
        self.is_output = is_output
        if seed >= 0:
            self.random_state = gmpy2.random_state(seed)
        else:
            self.random_state = gmpy2.random_state(int(time.time() * 10000000))
        self.prover = []
        self.verifier = []
        for i in range(self.func_nums + 1):
            self.prover.append(Prover()) #* (self.func_nums + 1)
            self.verifier.append(Verifier()) #* (self.func_nums + 1)
        self.func_a = []
        self.func_k = []
        self.func_sk = []
        self.func_pk = []
        self.witnesses = [self.witness]

    def get_func(self, ):
        _bit_len = self.bit_len
        for i in range(self.func_nums):
            _bit_len = _bit_len // 2
            temp = mpz(pow(2, _bit_len))
            temp2 = int(gmpy2.mpz_random(self.random_state,  temp))
            a = self.witness - temp2 
            while True:
                if a < self.witness and a > 0:
                    self.func_a.append(a)
                    self.func_k.append(_bit_len)
                    print("a: " + str(a))
                    print("temp2: " + str(temp2))
                    #print("pow: " + str(pow(2, _bit_len)))
                    break
                else:
                    temp2 = int(gmpy2.mpz_random(self.random_state,  temp))
                    a = self.witness - temp2 
        for i in range(self.func_nums):
            self.witnesses.append(self.witness - self.func_a[i])
        print(self.func_a)
        print(self.func_k)
        print("witnesses")
        print(self.witnesses)
        return        

    def keygen(self, ):
        self._p = gmpy2.mpz_urandomb(self.random_state, self.p_length)
        self._p = gmpy2.next_prime(self._p)
        self._q = self._p * 2 + 1
        while gmpy2.is_prime(self._q, self.prime_test_reps) != 1 :
            self._p = gmpy2.next_prime(self._p)
            self._q = self._p * 2 + 1
        def generator_yield():
            temp_generator = gmpy2.mpz_random(self.random_state, self._q)
            while True:
                temp1 = gmpy2.powmod(temp_generator, self._p, self._q)
                temp2 = gmpy2.powmod(temp_generator, 2, self._q)
                if temp1 == 1 and temp2 != 1 and temp_generator != self._p and temp_generator != 0:
                    break
                else:
                    temp_generator = gmpy2.mpz_random(self.random_state, self._q)
            return temp_generator
        self._g = int(generator_yield())
        self._h = int(generator_yield())
        self._u = int(generator_yield())
        for i in range(self.bit_len):
            self._vec_g.append(int(generator_yield()))
            self._vec_h.append(int(generator_yield()))
        # set parameters 
        _bit_len = self.bit_len
        for i in range(self.func_nums + 1):
            
            self.prover[i]._p = self.verifier[i]._p = int(self._p)
            self.prover[i]._q = self.verifier[i]._q = int(self._q)

            self.prover[i]._g = self.verifier[i]._g = self._g 
            self.prover[i]._h = self.verifier[i]._h = self._h
            self.prover[i]._u = self.verifier[i]._u = self._u

            self.prover[i]._vec_g = self._vec_g[:_bit_len]
            self.prover[i]._vec_h = self._vec_h[:_bit_len]
            self.verifier[i]._vec_g = self._vec_g[:_bit_len]
            self.verifier[i]._vec_h = self._vec_h[:_bit_len]

            self.prover[i].bit_len = self.verifier[i].bit_len = _bit_len #self.bit_len 
            print("prover bit len 1: " + str(self.prover[i].bit_len))
            self.prover[i].random_state = self.random_state
            self.verifier[i].random_state = self.random_state

            print("_bit Len: " + str(_bit_len))
            print("prover bit len 2: " + str(self.prover[i].bit_len))
            _bit_len = _bit_len // 2 

        for i in range(self.func_nums + 1):
            print("prover bit len: " + str(self.prover[i].bit_len))
        for i in range(self.func_nums):
            self.func_sk.append(int(gmpy2.mpz_random(self.random_state, self._p)))
        temp_pk = 1
        for i in range(self.func_nums):
           #temp_pk *= self.func_sk[i]
           temp_pk = pow(self._h, pow(self.func_sk[i] , -1, self._p), self._q)
           self.func_pk.append(temp_pk)
        #self.func_pk = [int(pow(self._h, pow(l, -1, self._q), self._q)) for l in self.func_sk]
        temp_pk = 1
        for i in range(self.func_nums):
            temp_pk *= self.func_pk[i] 
        self.func_pk.insert(0, temp_pk % self._q )
    
        self._pp = int(self._p)
        self._qq = int(self._q)
        if self.is_output:
            print("_p: " + str(self._p))
            print("_q: " + str(self._q))

            print("_g: " + str(self._g))
            print("_h: " + str(self._h))
            print("_u: " + str(self._u))
            print("_vec_g: ")
            print(self._vec_g)
            print("_vec_h: ")
            print(self._vec_h)
            print("func sk")
            print(self.func_sk)
            print("func pk")
            print(self.func_pk)

    def verify_protocal_fnizk_t(self, ):
        pass

    def run_protocal_fnizk(self, ):
        #for i in range(self.func_nums + 1):
        #    self.run_protocal_range_proof(i)
        self.run_protocal_range_proof()
        self.verify_protocal_fnizk_t()

    def run_protocal_range_proof(self, ):
        A_and_S = ""
        for i in range(self.func_nums + 1):
            print("a: " + str(self.prover[i].bit_len))
            self.prover[i].range_proof_before_yz(self.witnesses[i], self.is_output, self.func_pk[i])
            print("b: " + str(self.prover[i].bit_len))
            A_and_S += hex(self.prover[i]._A)[2:] + hex(self.prover[i]._S)[2:]
        _random_y = int(hashlib.sha256(A_and_S.encode('utf-8')).hexdigest(), 16) % self._pp
        A_and_S_and_y = A_and_S + hex(_random_y)[2:]
        _random_z = int(hashlib.sha256(A_and_S_and_y.encode('utf-8')).hexdigest(), 16) % self._pp
        T1_and_T2 = ""
        for i in range(self.func_nums + 1):
            self.prover[i]._random_y = _random_y
            self.prover[i]._random_z = _random_z
            self.prover[i].range_proof_after_yz(self.func_pk[i], self.func_nums , i)
            print("c: " + str(self.prover[i].bit_len))
            T1_and_T2 += hex(self.prover[i]._T1)[2:] + hex(self.prover[i]._T2)[2:]
        _random_x = int(hashlib.sha256(T1_and_T2.encode('utf-8')).hexdigest(), 16) % self._pp

        for i in range(self.func_nums + 1):
            self.prover[i]._random_x = _random_x
            self.prover[i].range_proof_after_T()
            print("d: " + str(self.prover[i].bit_len))

            print("_A, _S: " + str(i))
            print(self.prover[i]._A)
            print(self.prover[i]._S)

        for inxs in range(self.func_nums + 1):
            self.verifier[inxs]._random_x = _random_x
            self.verifier[inxs]._random_y = _random_y
            self.verifier[inxs]._random_z = _random_z
            res_rf = self.verifier[inxs].verifier_range_proof(self.prover[inxs]._tau_x, self.prover[inxs]._mu, self.prover[inxs]._hat_t, self.prover[inxs]._V, self.prover[inxs]._A, self.prover[inxs]._S, self.prover[inxs]._T1, self.prover[inxs]._T2, self.is_output)
        for inxs in range(self.func_nums + 1):
            self.prover[inxs]._c = self.prover[inxs]._hat_t 
            self.verifier[inxs]._c = self.prover[inxs]._hat_t
            self.prover[inxs]._P = self.verifier[inxs]._P
            self.prover[inxs]._vec_h = self.verifier[inxs]._vec_h[:]
            
        res_ip = self.run_protocal_inner_product_argument()
        print("res ip : " + str(res_ip))
        res_syn = self.verify_synthesis(_random_x, _random_y, _random_z)
        print("res syn: " + str(res_syn))
        if res_syn and res_ip:
            print("verifier accept!!!")
        else:
            print("verifier reject!!!")
    
    def verify_synthesis(self, _random_x, _random_y, _random_z):
        deltas = []
        for i in range(self.func_nums + 1):
            deltas.append(get_delta(_random_z, _random_y, self.prover[i].bit_len , self._pp))
        _m = self.func_nums + 1
        # test 
        
        _sum_t = 0
        for i in range(self.func_nums):
            _sum_t += self.prover[i+1]._hat_t
        _sum_ind = self.func_nums * self.prover[0]._hat_t - _sum_t 
        _res_1 = pow(self._g, _sum_ind, self._q)
        _mul_dot = 1
        for i in range(self.func_nums):
            _mul_dot *= pow(self._g, self.prover[0]._hat_t - self.prover[i+1]._hat_t, self._q)
        print("_res_1:   " + str(_res_1))
        print("_mul_dot: " + str(_mul_dot%self._q))

        pks = 1
        for i in range(self.func_nums):
            pks *= self.func_pk[i+1]
        pks = pks % self._qq
        _hl = pow(pks * pow(self._h, self.func_nums, self._qq) % self._qq, self.prover[0]._tau_x, self._qq)
        _hr = 1
        for i in range(self.func_nums):
            _htemp = pow(self._h * self.func_pk[i+1], self.prover[i+1]._tau_x, self._qq)
            _hr *= pow(_htemp, -1, self._qq)
        print("_hl*_hr: " + str(_hl*_hr%self._qq))
        
        _hres = 1
        for i in range(self.func_nums):
            _hres *= pow(self._h * self.func_pk[i+1], self.prover[0]._tau_x - self.prover[i+1]._tau_x, self._qq)
        print("_hres : " + str(_hres % self._qq))
        # test end 

        temp1 = 0
        for i in range(self.func_nums):
            temp1 += self.prover[i+1]._hat_t
        temp1 = self.func_nums * self.prover[0]._hat_t - temp1
        temp1 = pow(self._g, temp1, self._qq ) 
        temp2 = 1
        for i in range(self.func_nums):
            temp2 *= self.func_pk[i+1]
        temp2 = pow(temp2 * pow(self._h, self.func_nums, self._qq) % self._qq, self.prover[0]._tau_x , self._qq)
        temp3 = 1
        for i in range(self.func_nums):
            temp_ = pow(self._h * self.func_pk[i+1], self.prover[i+1]._tau_x, self._qq)
            temp3 *= pow(temp_, -1, self._qq)
        check_l = temp1 * temp2 * temp3 % self._qq
        
        temp1 = 1
        for i in range(self.func_nums):
            temp1 *= pow(self.func_pk[i+1], self.prover[0]._gamma - self.prover[i+1]._gamma, self._qq)
        temp1 *= pow(self.prover[0]._V, self.func_nums, self._qq) % self._qq
        temp2 = 1
        for i in range(self.func_nums):
            temp2 *= pow(self.prover[i+1]._V, -1, self._qq) 

        temp1 = pow(temp1 * temp2 % self._qq, _random_z * _random_z, self._qq )
        temp2 = 0
        for i in range(self.func_nums):
            temp2 += get_delta(_random_z, _random_y, self.prover[i+1].bit_len , self._pp)
        temp2 = self.func_nums * get_delta(_random_z, _random_y, self.prover[0].bit_len, self._pp) - temp2
        temp2 = pow(self._g, temp2, self._qq)
        temp3 = 1
        for i in range(self.func_nums):
            temp3 *= self.prover[i+1]._T1 
        #temp3 = pow(temp3, -1, self._qq) * pow(self.prover[0]._T1 , _m , self._qq) % self._qq
        temp3 = pow(temp3, -1, self._qq) * self.prover[0]._T1 % self._qq
        temp3 = pow(temp3, _random_x, self._qq)
        temp4 = 1
        for i in range(self.func_nums):
            temp4 *= self.prover[i+1]._T2 
        #temp4 = pow(temp4, -1, self._qq) * pow(self.prover[0]._T2 , _m , self._qq) % self._qq
        temp4 = pow(temp4, -1, self._qq) * self.prover[0]._T2 % self._qq
        temp4 = pow(temp4, _random_x * _random_x, self._qq)

        check_r = temp1 * temp2 * temp3 * temp4 % self._qq

        print("check_l: " + str(check_l))
        print("check_r: " + str(check_r))
        temp1 = 1
        for i in range(self.func_nums):
            temp1 *= pow(self._g, self.prover[0]._hat_t - self.prover[i+1]._hat_t, self._qq) * pow(self._h * self.func_pk[i+1], self.prover[0]._tau_x - self.prover[i+1]._tau_x, self._qq)
        print("check 2: " + str(temp1%self._qq))
        
        ln = 1
        temp_l3 = 1
        for i in range(self.func_nums):
            alphas = pow(self.func_pk[i+1], self.prover[0]._gamma - self.prover[i+1]._gamma, self._qq)
            vi = pow(self.prover[i+1]._V, -1, self._qq)
            l1 = pow(alphas * self.prover[0]._V * vi, _random_z * _random_z, self._qq)
            l2 = pow(self._g, self.verifier[0].delta - self.verifier[i+1].delta, self._qq)
            gs1 = pow(self._g, self.prover[0]._t1, self._qq)
            hs1 = pow(self._h * self.func_pk[i+1], self.prover[0]._tau1, self._qq)
            ts1 = pow(self.prover[i+1]._T1, -1, self._qq)
            l3 = pow(gs1 * hs1 * ts1 , _random_x, self._qq)
            temp_l3 *= l3
            gs2 = pow(self._g, self.prover[0]._t2, self._qq)
            hs2 = pow(self._h * self.func_pk[i+1], self.prover[0]._tau2, self._qq)
            ts2 = pow(self.prover[i+1]._T2, -1, self._qq)
            l4  = pow(gs2 * hs2 * ts2 , _random_x*_random_x, self._qq)
            ln = ln * l1 * l2 * l3 * l4 % self._qq
        print("ln: " + str(ln)) # third line
        print("temp_l3: " + str(temp_l3 % self._qq))

        # again
        alphas = 1
        for i in range(self.func_nums):
            alphas *= pow(self.func_pk[i+1], self.prover[0]._gamma - self.prover[i+1]._gamma, self._qq)
        v0 = pow(self.prover[0]._V, self.func_nums, self._qq)
        vs = 1
        for i in range(self.func_nums):
            vs *= pow(self.prover[i+1]._V, -1, self._qq)
        l1 = pow(alphas * v0 * vs, _random_z * _random_z, self._qq)
        l2 = pow(self._g, self.func_nums * self.verifier[0].delta - sum([self.verifier[i+1].delta for i in range(self.func_nums)]), self._qq)
        ts = 1
        for i in range(self.func_nums):
            ts *= self.prover[i+1]._T1 
        l3 = pow(pow(ts, -1, self._qq) * self.prover[0]._T1, _random_x, self._qq)
        ts2 = 1
        for i in range(self.func_nums):
            ts2 *= self.prover[i+1]._T2
        l4 = pow(pow(ts2, -1, self._qq) * self.prover[0]._T2, _random_x * _random_x, self._qq)
        ls = l1 * l2 * l3 * l4 % self._qq
        print("ls: " + str(ls)) # forth line

        nt1 = 1
        for i in range(self.func_nums):
            nt1 *= pow(self.prover[i+1]._T1, -1, self._qq)
        nt1 = pow(self.prover[0]._T1 * nt1 , _random_x, self._qq)
        print("temp nt3: " + str(nt1))
        nt2 = 1
        for i in range(self.func_nums):
            nt2 *= self.prover[i+1]._T1
        nt2 = pow(nt2, -1, self._qq)
        nt2 = pow(self.prover[0]._T1 * nt2 , _random_x, self._qq)
        print("nt1: " + str(nt1))
        print("nt2: " + str(nt2))

        assert(check_l == check_r)

        return check_l == check_r

    def run_protocal_inner_product_argument(self, ):
        _inner_res = 1
        for i in range(self.func_nums + 1):
            self.prover[i].inner_product_argument()
            print("prover g0: " + str(self.prover[i]._vec_g[0]));
            print("prover h0: " + str(self.prover[i]._vec_h[0]));
            _inner_= self.verifier[i].verifier_inner_product_argument(self.prover[i]._vec_g[0], self.prover[i]._vec_h[0], self.prover[i]._vec_a[0], self.prover[i]._vec_b[0], self.prover[i]._array_l, self.prover[i]._array_r)
            _inner_res &= _inner_
            print("inx: " + str(i)+" res: "+str(_inner_))
        return _inner_res

if __name__ == '__main__':
    p_length = 64
    witness = 150
    bit_len = 8
    func_nums = 2
    fnizk = FNIZK(p_length, witness, bit_len, func_nums, seed = 10, is_output = 1)
    fnizk.get_func()
    fnizk.keygen()
    fnizk.run_protocal_fnizk()
    #fnizk.run_fnizk()
    #fnizk.run_protocal_range_proof()
