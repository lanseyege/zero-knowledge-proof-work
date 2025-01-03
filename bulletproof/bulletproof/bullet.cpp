#include "bullet.h"

BulletProof::BulletProof() {
    prime_test_reps = 40;
    _time = clock();
    gmp_randinit_default(_gstate);
    //gmp_randseed_ui(_gstate, _time);
    gmp_randseed_ui(_gstate, 10);

    mpz_init(_dl_p_key);
    mpz_init(_dl_p2_key);
    mpz_init(_dl_q_key);
    mpz_init(_dl_g_key);
    mpz_init(_dl_h_key);

    mpz_init(_P);
    mpz_init(_u);
    mpz_init(_c);

}

BulletProof::~BulletProof() {
    mpz_clear(_dl_p_key);
    mpz_clear(_dl_p2_key);
    mpz_clear(_dl_q_key);
    mpz_clear(_dl_g_key);
    mpz_clear(_dl_h_key);

    mpz_clear(_P);
    mpz_clear(_u);
    mpz_clear(_c);

    for (int i = 0; i < bit_len; i ++) {
        mpz_clear(_vector_g[i]);
        mpz_clear(_vector_h[i]);
    }
}

void BulletProof::setup(int _bit_len, int _witness) {
    bit_len = _bit_len; witness = _witness;
    _vector_g = new mpz_t[bit_len];
    _vector_h = new mpz_t[bit_len];

    for (int i = 0; i < bit_len; i ++) {
        mpz_init(_vector_g[i]);
        mpz_init(_vector_h[i]);
    }
}

void BulletProof::keygen(int p_length, int is_output) {
    this->is_output = is_output;
    mpz_t _temp_p;
    mpz_t _temp_1;
    mpz_t _temp_2;

    mpz_init(_temp_p);
    mpz_init(_temp_1);
    mpz_init(_temp_2);
    cout << "a1" << endl;
    mpz_urandomb(_dl_p_key, _gstate, p_length);
    cout << "a1" << endl;
    mpz_nextprime(_dl_p_key, _dl_p_key);
    mpz_mul_ui(_dl_q_key, _dl_p_key, 2);
    mpz_add_ui(_dl_q_key, _dl_q_key, 1);

    cout << "a2" << endl;
    while(mpz_probab_prime_p(_dl_q_key, prime_test_reps) != 1) {
        mpz_nextprime(_dl_p_key, _dl_p_key);
        mpz_mul_ui(_dl_q_key, _dl_p_key, 2);
        mpz_add_ui(_dl_q_key, _dl_q_key, 1);
    }
    //mpz_sub_ui(_dl_p2_key, _dl_p_key, 1);
    mpz_urandomm(_dl_g_key, _gstate, _dl_q_key);

    cout << "a3" << endl;
    while(1) {
        mpz_powm(_temp_1, _dl_g_key, _dl_p_key,  _dl_q_key);
        mpz_powm_ui(_temp_2, _dl_g_key, 2, _dl_q_key);
        if (mpz_cmp_ui(_temp_1, 1) == 0 && mpz_cmp_ui(_temp_2, 1) != 0 && mpz_cmp(_dl_g_key, _dl_p_key) != 0) {
        //if (mpz_cmp_ui(_temp_1, 1) == 0)
            break;
        }
        else{
            //srand(time(0));
            //seed = rand();
            mpz_urandomm(_dl_g_key, _gstate, _dl_q_key);
        }
    }

    mpz_urandomm(_dl_h_key, _gstate, _dl_q_key);

    cout << "a4" << endl;
    while(1) {
        mpz_powm(_temp_1, _dl_h_key, _dl_p_key,  _dl_q_key);
        mpz_powm_ui(_temp_2, _dl_h_key, 2, _dl_q_key);
        if (mpz_cmp_ui(_temp_1, 1) == 0 && mpz_cmp_ui(_temp_2, 1) != 0 && mpz_cmp(_dl_h_key, _dl_p_key) != 0) {
            break;
        }
        else{
            mpz_urandomm(_dl_h_key, _gstate, _dl_q_key);
        }
    }
    mpz_urandomm(_u, _gstate, _dl_q_key);

    while(1) {
        mpz_powm(_temp_1, _u, _dl_p_key,  _dl_q_key);
        mpz_powm_ui(_temp_2, _u, 2, _dl_q_key);
        if (mpz_cmp_ui(_temp_1, 1) == 0 && mpz_cmp_ui(_temp_2, 1) != 0 && mpz_cmp(_u, _dl_p_key) != 0) {
            break;
        }
        else{
            mpz_urandomm(_u, _gstate, _dl_q_key);
        }
    }

    cout << "a5" << endl;
    for (int i = 0; i < bit_len; i++) {
        mpz_urandomm(_vector_g[i], _gstate, _dl_q_key);
        while(1) {
            mpz_powm(_temp_1, _vector_g[i], _dl_p_key,  _dl_q_key);
            mpz_powm_ui(_temp_2, _vector_g[i], 2, _dl_q_key);
            if (mpz_cmp_ui(_temp_1, 1) == 0 && mpz_cmp_ui(_temp_2, 1) != 0 && mpz_cmp(_vector_g[i], _dl_p_key) != 0) {
                break;
            }
            else{
                mpz_urandomm(_vector_g[i], _gstate, _dl_q_key);
            }
        }

        mpz_urandomm(_vector_h[i], _gstate, _dl_q_key);
        while(1) {
            mpz_powm(_temp_1, _vector_h[i], _dl_p_key,  _dl_q_key);
            mpz_powm_ui(_temp_2, _vector_h[i], 2, _dl_q_key);
            if (mpz_cmp_ui(_temp_1, 1) == 0 && mpz_cmp_ui(_temp_2, 1) != 0 && mpz_cmp(_vector_h[i], _dl_p_key) != 0) {
                break;
            }
            else{
                mpz_urandomm(_vector_h[i], _gstate, _dl_q_key);
            }
        }
    }

    if (is_output) {
        gmp_printf("p: %Zd\n", _dl_p_key);
        //gmp_printf("p2: %Zd\n", _dl_p2_key);
        gmp_printf("q: %Zd\n", _dl_q_key);
        gmp_printf("g: %Zd\n", _dl_g_key);
        gmp_printf("h: %Zd\n", _dl_h_key);
        gmp_printf("u: %Zd\n", _u);
        //mpz_sub_ui(_temp_2, _dl_p_key, 1);
        //mpz_powm(_temp_1, _dl_g_key, _temp_2, _dl_p_key);
        //gmp_printf("temp g: %Zd\n", _temp_1);
        //mpz_powm(_temp_1, _dl_h_key, _temp_2, _dl_p_key);
        //gmp_printf("temp h: %Zd\n", _temp_1);
        for (int i = 0; i < bit_len; i ++) {
            gmp_printf("vector g: %Zd\n", _vector_g[i]);
            //mpz_powm(_temp_1, _vector_g[i], _temp_2, _dl_p_key);
            //gmp_printf("g_i ^ p-1: %Zd\n", _temp_1);

            gmp_printf("vector h: %Zd\n", _vector_h[i]);
            //mpz_powm(_temp_1, _vector_h[i], _temp_2, _dl_p_key);
            //gmp_printf("h_i ^ p-1: %Zd\n", _temp_1);

        }
    }

    mpz_clear(_temp_p);
    mpz_clear(_temp_1);
    mpz_clear(_temp_2);
}

void BulletProof::test_hat_t(mpz_t _t1, mpz_t _t2, mpz_t _tau1, mpz_t _tau2, mpz_t _gamma, mpz_t _x, mpz_t _y, mpz_t _z, mpz_t _delta, mpz_t _rhat_t) {
    gmp_printf("tau1 : %Zd\n", _tau1);
    gmp_printf("tau2 : %Zd\n", _tau2);
    gmp_printf("z : %Zd\n", _z);
    gmp_printf("x : %Zd\n", _x);
    gmp_printf("gamma : %Zd\n", _gamma);

    mpz_t v; mpz_init(v);
    mpz_set_ui(v, 44);
    mpz_t _temp1; mpz_init(_temp1);   
    mpz_t _temp2; mpz_init(_temp2); 

    mpz_mul(_temp1, _z, _z);
    mpz_mod(_temp1, _temp1, _dl_p_key);
    mpz_mul(_temp1, v, _temp1);
    mpz_add(_temp1, _temp1, _delta);
    mpz_mod(_temp1, _temp1, _dl_p_key);
    gmp_printf("t0 : %Zd\n", _temp1);
    mpz_mul(_temp2, _t1, _x);
    mpz_mod(_temp2, _temp2, _dl_p_key);
    mpz_add(_temp1, _temp1, _temp2);
    mpz_mul(_temp2, _x, _x);
    mpz_mod(_temp2, _temp2, _dl_p_key);

    mpz_mul(_temp2, _t2, _temp2);
    mpz_mod(_temp2, _temp2, _dl_p_key);
    mpz_add(_temp1, _temp1, _temp2);
    mpz_mod(_temp1, _temp1, _dl_p_key);
    gmp_printf("hat t:      %Zd\n", _temp1);
    gmp_printf("real hat t: %Zd\n", _rhat_t);

    
    mpz_mul(_temp1, _z, _z);
    mpz_mul(_temp1, _temp1, _gamma);
    mpz_mul(_temp2, _tau1, _x);
    mpz_add(_temp1, _temp1, _temp2);
    mpz_mul(_temp2, _x, _x);
    mpz_mul(_temp2, _temp2, _tau2);
    mpz_add(_temp1, _temp1, _temp2);
    mpz_mod(_temp1, _temp1, _dl_p_key);
    gmp_printf("tau x: %Zd\n", _temp1);


    mpz_clear(_temp1);  
    mpz_clear(_temp2);
}

void BulletProof::run_protocal_range_proof() {

    //mpz_t _random_x; mpz_init(_random_x);
    //mpz_t _P_prime; mpz_init(_P_prime);
    //mpz_t _temp_1; mpz_init(_temp_1);
    prover.set_public(_dl_p_key, _dl_q_key, _dl_g_key, _dl_h_key, _u, _vector_g, _vector_h, bit_len );
    prover.setup_vector_ab();
    verifier.set_public(_dl_p_key, _dl_q_key, _dl_g_key, _dl_h_key, _u, _vector_g, _vector_h, bit_len );
    prover.prove_compute_proof_range_proof(witness, 1);
    if (is_output) {
        gmp_printf("_vector_a 0: %Zd\n", prover._vector_a[0]);
        gmp_printf("_vector_b 0: %Zd\n", prover._vector_b[0]);
        gmp_printf("_vector_g 0: %Zd\n", prover._vector_g[0]);
        gmp_printf("_vector_h 0: %Zd\n", prover._vector_h[0]);
    }
    int _verifier_rp_1 = verifier.verify_compute_proof_range_proof(prover._tau_x, prover._mu, prover._hat_t, prover._V, prover._A, prover._S, prover._T1, prover._T2 ) ; // prover._vector_g[0], prover._vector_h[0], prover._P, prover._array_l, prover._array_r
    if ( ! _verifier_rp_1 ) {
        cout << " range proof verifier failed !!" << endl;
        return;
    }
 
    //test_hat_t(prover._test_t1, prover._test_t2, prover._test_tau1, prover._test_tau2, prover._test_gamma, prover._test_x, prover._test_y, prover._test_z, verifier._test_delta_yz, prover._hat_t);

    mpz_set(_c, prover._hat_t);
    mpz_set(prover._c, prover._hat_t);
    mpz_set(verifier._c, prover._hat_t);
    mpz_set(_P, verifier._P);
    mpz_set(prover._P, verifier._P);
    for (int i = 0; i < bit_len; i ++) {
        mpz_set(prover._vector_h[i] , verifier._vector_h[i]);
    }
    // test P_R:
    prover.test_pr();
    int _verifier_ip_1 = run_protocal_inner_product_argument(1);
    if (_verifier_rp_1 && _verifier_ip_1) {
        cout << "verifier accept!!!" << endl;
    } else {
        cout << "verifier reject!!!" << endl;
    }
    return ;

}


int BulletProof::run_protocal_inner_product_argument(int flag) {
    prover.prove_proof_inner_product(flag);
    gmp_printf("prover_p0: %Zd\n", prover._P);
    return verifier.verify_compute_proof_inner_product(prover._vector_g[0], prover._vector_h[0], prover._vector_a[0], prover._vector_b[0], prover._P, prover._u, prover._array_l, prover._array_r);
}
