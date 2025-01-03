import gmpy2
from gmpy2 import mpz
import time
import hashlib

from utils import *
from prover import Prover
from verifier import Verifier

class BulletProof():
    def __init__(self, p_length, witness, bit_len, seed, is_output):
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
        self.is_output = is_output
        if seed >= 0:
            self.random_state = gmpy2.random_state(seed)
        else:
            self.random_state = gmpy2.random_state(int(time.time() * 10000000))

        self.prover = Prover()
        self.verifier = Verifier()


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

        self.prover._p = self.verifier._p = int(self._p)
        self.prover._q = self.verifier._q = int(self._q)

        self.prover._g = self.verifier._g = self._g 
        self.prover._h = self.verifier._h = self._h
        self.prover._u = self.verifier._u = self._u

        self.prover._vec_g = self._vec_g[:]
        self.prover._vec_h = self._vec_h[:]
        self.verifier._vec_g = self._vec_g[:]
        self.verifier._vec_h = self._vec_h[:]

        self.prover.bit_len = self.verifier.bit_len = self.bit_len 
        self.prover.random_state = self.random_state
        self.verifier.random_state = self.random_state

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


    def run_protocal_range_proof(self, ):
        self.prover.range_proof(self.witness, self.is_output)
        print("_A, _S")
        print(self.prover._A)
        print(self.prover._S)
        res_rf = self.verifier.verifier_range_proof(self.prover._tau_x, self.prover._mu, self.prover._hat_t, self.prover._V, self.prover._A, self.prover._S, self.prover._T1, self.prover._T2, self.is_output)
        self.prover._c = self.prover._hat_t 
        self.verifier._c = self.prover._hat_t
        self.prover._P = self.verifier._P
        self.prover._vec_h = self.verifier._vec_h[:]
        res_ip = self.run_protocal_inner_product_argument()
        if res_rf and res_ip:
            print("verifier accept!!!")
        else:
            print("verifier reject!!!")

    def run_protocal_inner_product_argument(self, ):
        self.prover.inner_product_argument()
        print("prover g0: " + str(self.prover._vec_g[0]));
        print("prover h0: " + str(self.prover._vec_h[0]));
        return self.verifier.verifier_inner_product_argument(self.prover._vec_g[0], self.prover._vec_h[0], self.prover._vec_a[0], self.prover._vec_b[0], self.prover._array_l, self.prover._array_r)


if __name__ == '__main__':
    p_length = 1024
    witness = 50
    bit_len = 8
    bulletproof = BulletProof(p_length, witness, bit_len, seed = 10, is_output = 1)
    bulletproof.keygen()
    bulletproof.run_protocal_range_proof()
