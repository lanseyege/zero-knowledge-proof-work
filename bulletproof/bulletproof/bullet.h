#include <iostream>
#include <stdio.h>
#include <math.h>
#include <gmpxx.h>
#include <time.h>
#include <stdlib.h>

#include "basic.h"

using namespace std;

class BulletProof {
    
    public :
        BulletProof();
        ~BulletProof();

        void setup(int _bit_len, int _witness);
        void keygen(int p_length, int is_output);
        void run_protocal_range_proof();
        int run_protocal_inner_product_argument(int flag);

        void test_hat_t(mpz_t _t1, mpz_t _t2, mpz_t _tau1, mpz_t _tau2, mpz_t _gamma, mpz_t _x, mpz_t _y, mpz_t _z, mpz_t _delta, mpz_t _rhat_t) ;

    private:

        mpz_t _dl_p_key;
        mpz_t _dl_p2_key; // p - 1
        mpz_t _dl_q_key;
        mpz_t _dl_g_key;
        mpz_t _dl_h_key;

        mpz_t *_vector_g, *_vector_h;

        mpz_t _P, _u, _c;

        gmp_randstate_t _gstate;
        clock_t _time;

        int prime_test_reps;

        int bit_len;
        int witness;
        int is_output;
        
        Prover prover;
        Verifier verifier;
};
