#include <iostream>
#include <stdio.h>
#include <math.h>
#include <gmpxx.h>
#include <time.h>
#include <stdlib.h>
#include <cryptopp/sha.h>
#include <cryptopp/hex.h>
#include <cryptopp/filters.h>
#include <cryptopp/files.h>

using namespace std;
using namespace CryptoPP;

class Basic {

    public :
        Basic();
        ~Basic();

        void set_public(mpz_t m_dl_p_key, mpz_t m_dl_q_key, mpz_t m_dl_g_key, mpz_t m_dl_h_key, mpz_t _u, mpz_t *m_vector_g, mpz_t *m_vector_h, int _bit_len) ;
        void generate_random(string msg, mpz_t _random);
        void pedersen_commitment_scalar(mpz_t _base1, mpz_t _inx1, mpz_t _base2, mpz_t _inx2, mpz_t _result, mpz_t _temp) ;
        void pedersen_commitment_vector(mpz_t *_base1, int _start_b1, mpz_t *_inx1, int _start_inx1, mpz_t *_base2, int _start_b2, mpz_t *_inx2, int _start_inx2, mpz_t _result, int _len, mpz_t _temp);
        void hadamard_product(mpz_t *_left , int _left_start, mpz_t *_right, int _right_start, mpz_t *_result, int _len, mpz_t _pkey) ;
        void inner_product(mpz_t *_left, int left_start, mpz_t *_right, int right_start, mpz_t _result, int _len, mpz_t _temp);
        void pow_scalar(mpz_t *_base, int _start, mpz_t index, int _len);
        void mul_scalar(mpz_t *_base, int _start, mpz_t _scalar, int _len) ;
        string get_msg_phug() ;
        string get_msg(mpz_t _t1, mpz_t _t2);
        string get_msg(mpz_t _t1, mpz_t _t2, mpz_t _t3);


        mpz_t _dl_p_key;
        mpz_t _dl_q_key;
        mpz_t _dl_g_key;
        mpz_t _dl_h_key;

        //mpz_t *_vector_a, *_vector_b;
        mpz_t *_vector_g, *_vector_h;

        mpz_t _P, _u, _c;

        gmp_randstate_t _gstate;
        clock_t _time;

        //int prime_test_reps;

        int bit_len, _logn;

        //int witness;
        SHA256 sha256;
    private:
        int __a1 = 1;
};

class Prover : public Basic {
    
    public :
        Prover();
        ~Prover();
        
        void setup_vector_ab();
        // prove functions
        void prove_compute_proof_inner_product(mpz_t c_L, mpz_t c_R, mpz_t L, mpz_t R, int _bit_len, mpz_t _temp, mpz_t _temp2, mpz_t _random_x, int i);
        void prove_proof_inner_product(int flag) ;
        void prove_compute_proof_range_proof(int witness, int is_output);
        void test_pr();

        //  variables transfer to verifier
        mpz_t _tau_x;
        mpz_t _mu;
        mpz_t _hat_t;
        mpz_t _V;
        mpz_t _A;
        mpz_t _S;
        mpz_t _T1;
        mpz_t _T2;
        //mpz_t _P;
        mpz_t *_vector_a, *_vector_b;
        mpz_t *_array_l, *_array_r; // store L and R in recursive process
        mpz_t _test_tau1, _test_tau2;
        mpz_t _test_t1, _test_t2;
        mpz_t _test_x, _test_y, _test_z;
        mpz_t _test_gamma;

    private:
        void get_t1(mpz_t *_al, mpz_t *_ar, mpz_t *_SL, mpz_t *_SR, mpz_t _random_y, mpz_t _random_z, mpz_t _temp_var1, mpz_t _temp_var2, mpz_t _result_l, mpz_t _result_r);
        void get_t2(mpz_t *_SL, mpz_t *_SR, mpz_t _random_y, mpz_t _temp_var1, mpz_t _result);
        void get_lx(mpz_t *_al, mpz_t *_SL, mpz_t _random_x, mpz_t _random_z, mpz_t *_lx, mpz_t _temp_var1);
        void get_rx(mpz_t *_ar, mpz_t *_SR, mpz_t _random_x, mpz_t _random_y, mpz_t _random_z, mpz_t *_rx, mpz_t _temp_var1, mpz_t _temp_var2);
        void get_tau_x(mpz_t _tau1, mpz_t _tau2, mpz_t _random_x, mpz_t _random_z, mpz_t _gamma, mpz_t _taux, mpz_t _temp_var1);
        void get_mu(mpz_t _alpha, mpz_t _rho, mpz_t _random_x, mpz_t _mu);

};

class Verifier : public Basic {
    public :
        Verifier();
        ~Verifier();

        mpz_t _test_delta_yz;

        int verify_compute_proof_range_proof(mpz_t _tau_x, mpz_t _mu, mpz_t _hat_t, mpz_t _V, mpz_t _A, mpz_t _S, mpz_t _T1, mpz_t _T2);
        int verify_compute_proof_inner_product(mpz_t _vector_g0, mpz_t  _vector_h0, mpz_t _vector_a0, mpz_t _vector_b0, mpz_t _P0, mpz_t m_u, mpz_t *_arry_l, mpz_t *_array_r);

    private:
        int __a2 = 1;
};


