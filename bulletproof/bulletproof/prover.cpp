#include "basic.h"

Prover::Prover() {
    mpz_init(_tau_x);
    mpz_init(_mu);
    mpz_init(_hat_t);
    mpz_init(_V);
    mpz_init(_A);
    mpz_init(_S);
    mpz_init(_T1);
    mpz_init(_T2);
    mpz_init(_P);

    mpz_init(_test_tau1);
    mpz_init(_test_tau2);
    mpz_init(_test_t1);
    mpz_init(_test_t2);
    mpz_init(_test_gamma);
    mpz_init(_test_x);
    mpz_init(_test_y);
    mpz_init(_test_z);
}

Prover::~Prover() {
    mpz_clear(_tau_x);
    mpz_clear(_mu);
    mpz_clear(_hat_t);
    mpz_clear(_V);
    mpz_clear(_A);
    mpz_clear(_S);
    mpz_clear(_T1);
    mpz_clear(_T2);
    mpz_clear(_P);
    mpz_clear(_test_tau1);
    mpz_clear(_test_tau2);
    mpz_clear(_test_t1);
    mpz_clear(_test_t2);
    mpz_clear(_test_gamma);
    mpz_clear(_test_x);
    mpz_clear(_test_y);
    mpz_clear(_test_z);

    for (int i = 0; i < bit_len; i++) {
        mpz_clear(_vector_a[i]);
        mpz_clear(_vector_b[i]);
    }
}

void Prover::setup_vector_ab() {
    _vector_a = new mpz_t[bit_len];
    _vector_b = new mpz_t[bit_len];
    for (int i = 0; i < bit_len; i++) {
        mpz_init(_vector_a[i]);
        mpz_init(_vector_b[i]);
    }
}

void Prover::get_t1(mpz_t *_al, mpz_t *_ar, mpz_t *_SL, mpz_t *_SR, mpz_t _random_y, mpz_t _random_z, mpz_t _temp_var1, mpz_t _temp_var2, mpz_t _result_l, mpz_t _result_r) {
    mpz_set_ui(_result_l, 0) ;
    mpz_set_ui(_result_r, 0) ;
    mpz_t _temp_y; mpz_init(_temp_y); mpz_set_ui(_temp_y, 1);
    mpz_t _temp_tw; mpz_init(_temp_tw); mpz_set_ui(_temp_tw, 1);
    mpz_t _temp_z; mpz_init(_temp_z); mpz_mul(_temp_z, _random_z, _random_z);
    for (int i = 0; i < bit_len; i++) {

        // t1, right part of +
        mpz_add( _temp_var1, _random_z, _ar[i] ); // z + ar[i]
        //gmp_printf("z + ar: %Zd\n", _temp_var1);
        mpz_mul(_temp_var1 , _temp_y, _temp_var1); // y^i*(z+ar[i])
        mpz_mul(_temp_var2, _temp_tw, _temp_z);//y^i*(z+ar[i])+z^2*2^i
        mpz_add(_temp_var1, _temp_var1, _temp_var2);
        mpz_mul(_temp_var1 , _SL[i], _temp_var1);
        //gmp_printf("SL * y * -: %Zd\n", _temp_var1);
        mpz_add(_result_r , _result_r, _temp_var1);
        mpz_mod(_result_r, _result_r, _dl_p_key);
        // t1, left part of +
        mpz_mul(_temp_var1, _temp_y, _SR[i]);
        mpz_sub(_temp_var2, _al[i] , _random_z);
        //gmp_printf("al - z: %Zd\n", _temp_var2);
        //gmp_printf("y * SR: %Zd\n", _temp_var1);
        mpz_mul(_temp_var1, _temp_var1, _temp_var2);
        //gmp_printf("**: %Zd\n", _temp_var1);
        mpz_add(_result_l , _result_l, _temp_var1);
        mpz_mod(_result_l, _result_l, _dl_p_key);

        mpz_mul(_temp_y, _temp_y, _random_y); // y ^ i
        mpz_mod(_temp_y, _temp_y, _dl_p_key);
        mpz_mul_ui(_temp_tw, _temp_tw, 2);
        mpz_mod(_temp_tw, _temp_tw, _dl_p_key);
        
    }
    //R + L
    mpz_add(_result_r, _result_r , _result_l);
    mpz_mod(_result_r, _result_r, _dl_p_key);

    mpz_clear(_temp_y);
    mpz_clear(_temp_z);
    mpz_clear(_temp_tw);
}

void Prover::get_t2(mpz_t *_SL, mpz_t *_SR, mpz_t _random_y, mpz_t _temp_var1, mpz_t _result) {
    mpz_t _temp_y; mpz_init(_temp_y); mpz_set_ui(_temp_y, 1);
    for (int i = 0; i < bit_len; i++) {
        mpz_mul(_temp_var1, _temp_y, _SR[i]);
        mpz_mul(_temp_var1, _SL[i], _temp_var1);
        mpz_add(_result, _result, _temp_var1);
        mpz_mod(_result, _result, _dl_p_key);

        mpz_mul(_temp_y, _temp_y, _random_y);
        mpz_mod(_temp_y, _temp_y, _dl_p_key);
    }
    mpz_mod(_result, _result, _dl_p_key);
    mpz_clear(_temp_y);
}

void Prover::get_lx(mpz_t *_al, mpz_t *_SL, mpz_t _random_x, mpz_t _random_z, mpz_t *_lx, mpz_t _temp_var1) {
    for (int i = 0; i < bit_len; i++) {
        mpz_mul(_temp_var1 , _SL[i], _random_x); // S_L * x
        //mpz_mod(_temp_var1, _temp_var1, _dl_p_key);
        mpz_add(_temp_var1, _temp_var1, _al[i]); // 
        //gmp_printf("_temp_var1: %Zd\n", _temp_var1);
        mpz_sub(_lx[i], _temp_var1, _random_z);
        mpz_mod(_lx[i], _lx[i], _dl_p_key);
        //gmp_printf("lx i: %Zd\n", _lx[i]);
    }
}

void Prover::get_rx(mpz_t *_ar, mpz_t *_SR, mpz_t _random_x, mpz_t _random_y, mpz_t _random_z, mpz_t *_rx, mpz_t _temp_var1, mpz_t _temp_var2) {
    mpz_t _temp_y; mpz_init(_temp_y);
    mpz_set_ui(_temp_y, 1);
    mpz_t _temp_v2; mpz_init(_temp_v2);
    mpz_set_ui(_temp_v2, 1); // 2 ^ n
    mpz_t _temp_z; mpz_init(_temp_z);
    mpz_mul(_temp_z, _random_z, _random_z);
    //mpz_mod(_temp_z, _temp_z, _dl_p2_key); // z^2
    for (int i = 0; i < bit_len; i++) {

        mpz_mul(_temp_var1, _SR[i], _random_x); // S_r * x
        mpz_add(_temp_var1, _temp_var1, _random_z); // + z
        mpz_add(_temp_var1, _temp_var1, _ar[i]); // + a_r
        mpz_mul(_temp_var1, _temp_var1, _temp_y); // *y ^ {i - 1}

        //mpz_mul(_temp_var2, _random_z, _random_z);
        mpz_mul(_temp_var2, _temp_z, _temp_v2); // z^2 * 2 ^ {i - 1}
        mpz_add(_rx[i], _temp_var1, _temp_var2); // . + . 
        mpz_mod(_rx[i], _rx[i], _dl_p_key); // mod 

        mpz_mul(_temp_y, _temp_y, _random_y); // y * y ..
        mpz_mod(_temp_y, _temp_y, _dl_p_key); // mod
        mpz_mul_ui(_temp_v2, _temp_v2, 2); // 2 * 2 .. 
        mpz_mod(_temp_v2, _temp_v2, _dl_p_key);
    }
    mpz_clear(_temp_y);
    mpz_clear(_temp_v2);
    mpz_clear(_temp_z);
}

void Prover::get_tau_x(mpz_t _tau1, mpz_t _tau2, mpz_t _random_x, mpz_t _random_z, mpz_t _gamma, mpz_t _taux, mpz_t _temp_var1) {
    mpz_mul(_temp_var1, _random_x, _random_x); // x^2
    mpz_mod(_temp_var1, _temp_var1, _dl_p_key); // mod
    mpz_mul(_taux, _tau2, _temp_var1); // \tau_2 * x^2
    mpz_mul(_temp_var1, _tau1, _random_x); // \tau_1 * x
    mpz_add(_taux, _taux, _temp_var1);
    mpz_mul(_temp_var1, _random_z, _random_z); // z^2
    mpz_mul(_temp_var1, _temp_var1, _gamma); // z^2 * \gamma
    mpz_add(_taux, _taux, _temp_var1);
    mpz_mod(_taux, _taux, _dl_p_key);
}

void Prover::get_mu(mpz_t _alpha, mpz_t _rho, mpz_t _random_x, mpz_t _mu) {
    mpz_mul(_mu, _rho, _random_x);
    mpz_add(_mu, _mu, _alpha);
    mpz_mod(_mu, _mu, _dl_p_key);
}

void Prover::test_pr() {
    for (int i = 0 ; i < bit_len; i ++) {
        cout << "g hi , a, b: " << i << endl;
        gmp_printf("%Zd\n", _vector_g[i]);
        gmp_printf("%Zd\n", _vector_h[i]);
        gmp_printf("%Zd\n", _vector_a[i]);
        gmp_printf("%Zd\n", _vector_b[i]);
    }
    gmp_printf("-- mu  %Zd\n", _mu);
    mpz_t _temp_var1; mpz_init(_temp_var1);
    mpz_t _temp_var2; mpz_init(_temp_var2);
    Basic::pedersen_commitment_vector(_vector_g, 0, _vector_a, 0, _vector_h, 0, _vector_b, 0,  _temp_var1, bit_len, _temp_var2);
    mpz_powm(_temp_var2, _dl_h_key, _mu, _dl_p_key);
    mpz_mul(_temp_var1, _temp_var1, _temp_var2);
    mpz_mod(_temp_var1, _temp_var1, _dl_p_key);
    gmp_printf("test PR: %Zd\n", _temp_var1);
    mpz_clear(_temp_var1);
    mpz_clear(_temp_var2);
}

void Prover::prove_proof_inner_product(int flag ) {
    mpz_t _temp_var1; mpz_init(_temp_var1);
    mpz_t _temp_var2; mpz_init(_temp_var2);
    mpz_t _random_xx; mpz_init(_random_xx);
    if (flag) {// flag > 0 means this protocal is called, paramenter will be passed into 
        // _P, _c be gotten from other protocal
    }else {
        Basic::pedersen_commitment_vector(_vector_g, 0, _vector_a, 0, _vector_h, 0, _vector_b, 0, _P, bit_len, _temp_var1); // get _P , the commitment
    }
    gmp_printf("start1 _P: %Zd\n", _P);
    gmp_printf("start1 _u: %Zd\n", _u);
    gmp_printf("start1 _c: %Zd\n", _c);

    Basic::generate_random(Basic::get_msg_phug() , _random_xx); // get random x
    mpz_mod(_random_xx, _random_xx, _dl_p_key);
    cout << "msg hpug: " << Basic::get_msg_phug() << endl;
    gmp_printf("_random xx: %Zd\n", _random_xx);
    //mpz_mul(_temp_var1, _random_xx, _hat_t); // x * c
    mpz_mul(_temp_var1, _random_xx, _c); // x * c
    mpz_powm(_temp_var1 , _u, _temp_var1, _dl_q_key); // u^{x * c}
    mpz_mul(_P, _temp_var1, _P);
    mpz_mod(_P, _P, _dl_q_key);
    mpz_powm(_u, _u, _random_xx, _dl_q_key);
    gmp_printf("start2 _P: %Zd\n", _P);
    gmp_printf("start2 _u: %Zd\n", _u);

    //int _logn = (int)log(bit_len);
    _array_l = new mpz_t[_logn];
    _array_r = new mpz_t[_logn];
    for (int i = 0; i < _logn; i++) {
        mpz_init(_array_l[i]);
        mpz_init(_array_r[i]);
    }
    mpz_t c_L, c_R, L, R ;
    mpz_init(c_L); mpz_init(c_R); mpz_init(L); mpz_init(R);
    //mpz_init(_random_xx);
    prove_compute_proof_inner_product(c_L, c_R, L, R, bit_len, _temp_var1, _temp_var2, _random_xx, 0);
    mpz_clear(_temp_var1);
    mpz_clear(_temp_var2);
    mpz_clear(_random_xx);
}

void Prover::prove_compute_proof_inner_product(mpz_t c_L, mpz_t c_R, mpz_t L, mpz_t R, int _bit_len, mpz_t _temp, mpz_t _temp2, mpz_t _random_xx, int k) {
    cout << "_bit_len : " << _bit_len << endl; 
    if (_bit_len == 1) {
        mpz_t _ccc; mpz_init(_ccc);
        cout << endl;
        cout << "prover self verifier, can be passed" << endl;
        //Basic::pedersen_commitment_scalar(_dl_g_key, _vector_a[0], _dl_h_key, _vector_b[0], _temp, _temp2);
        Basic::pedersen_commitment_scalar(_vector_g[0], _vector_a[0], _vector_h[0], _vector_b[0], _temp, _temp2);
        mpz_mul(_ccc, _vector_a[0], _vector_b[0]);
        mpz_powm(_temp2, _u, _ccc, _dl_q_key);
        mpz_mul(_temp, _temp2, _temp);
        mpz_mod(_temp, _temp, _dl_q_key);
        gmp_printf("prove cal _P: %Zd\n ", _temp);
        mpz_clear(_ccc);
        cout << endl;
        return ;
    }
    _bit_len = (int) _bit_len / 2; 
    //cout << "e1 " << endl;
    Basic::inner_product(_vector_a, 0, _vector_b, _bit_len, c_L, _bit_len, _temp); // get C_L
    //cout << "e2 " << endl;
    Basic::inner_product(_vector_a, _bit_len, _vector_b, 0, c_R, _bit_len, _temp); // get C_R
    Basic::pedersen_commitment_vector(_vector_g, _bit_len, _vector_a, 0, _vector_h, 0, _vector_b, _bit_len, L, _bit_len, _temp); // get L
    mpz_powm(_temp, _u, c_L, _dl_q_key);
    mpz_mul(L, L, _temp);
    mpz_mod(L, L, _dl_q_key); // get L
    Basic::pedersen_commitment_vector(_vector_g, 0, _vector_a, _bit_len, _vector_h, _bit_len, _vector_b, 0, R, _bit_len, _temp);
    mpz_powm(_temp, _u, c_R, _dl_q_key);
    mpz_mul(R, R, _temp);
    mpz_mod(R, R, _dl_q_key); // get R
    mpz_set(_array_l[k], L);
    mpz_set(_array_r[k], R);

    Basic::generate_random(Basic::get_msg(L, R), _random_xx); // get random x
    mpz_mod(_random_xx, _random_xx, _dl_p_key);
    cout << "k : " << k << endl;
    gmp_printf("_array_l: %Zd\n", L);
    gmp_printf("_array_r: %Zd\n", R);
    gmp_printf("verifier random: %Zd\n", _random_xx);
    cout << " vector g : " << endl;
    for (int j = 0 ; j < bit_len;  j++) {
        gmp_printf("%Zd ", _vector_g[j]);
    } cout << endl;
    cout << " vector h : " << endl;
    for (int j = 0 ; j < bit_len;  j++) {
        gmp_printf("%Zd ", _vector_h[j]);
    } cout << endl;

    // step 29
    //mpz_mod(_random_xx, _random_xx, _dl_p2_key);
    mpz_invert(_temp , _random_xx, _dl_p_key);
    Basic::pow_scalar(_vector_g, 0, _temp, _bit_len);
    Basic::pow_scalar(_vector_g, _bit_len, _random_xx, _bit_len);
    Basic::hadamard_product(_vector_g, 0, _vector_g, _bit_len, _vector_g, _bit_len, _dl_q_key); // get g^\prime
    // step 30
    Basic::pow_scalar(_vector_h, 0, _random_xx, _bit_len);
    Basic::pow_scalar(_vector_h, _bit_len, _temp, _bit_len);
    Basic::hadamard_product(_vector_h, 0, _vector_h, _bit_len, _vector_h, _bit_len, _dl_q_key); // get h^\prime
    // step 31 
    mpz_mul(_temp, _random_xx, _random_xx); // x ^ 2
    //mpz_mod(_temp, _temp, _dl_p_key);
    mpz_powm(_temp, L, _temp, _dl_q_key); // L ^ {x^2}
    mpz_mul(_P, _temp, _P); // L ^ {x^2} * P
    mpz_mod(_P, _P, _dl_q_key); // 
    mpz_mul(_temp, _random_xx, _random_xx); // x^ 2
    mpz_invert(_temp , _temp, _dl_p_key); // x ^ {-2}
    mpz_powm(_temp, R, _temp, _dl_q_key); // R^ {x^-2}
    mpz_mul(_P, _P, _temp); // L ^ {x^2} * P * R^ {x^-2} 
    mpz_mod(_P, _P, _dl_q_key); // get P^\prime 
    gmp_printf("_P in : %Zd\n", _P);
    
    mpz_invert(_temp2, _random_xx, _dl_p_key); // x^-1
    for (int i = 0 ; i < _bit_len; i ++ ) {
        mpz_mul(_temp, _vector_a[i], _random_xx);
        mpz_mul(_vector_a[i +_bit_len], _vector_a[i+_bit_len], _temp2);
        mpz_add(_vector_a[i], _temp,  _vector_a[i+_bit_len] );
        mpz_mod(_vector_a[i], _vector_a[i], _dl_p_key); // a^\prime

        mpz_mul(_temp, _vector_b[i], _temp2);
        mpz_mul(_vector_b[i+_bit_len], _vector_b[i+_bit_len], _random_xx);
        mpz_add(_vector_b[i], _temp, _vector_b[i+_bit_len]);
        mpz_mod(_vector_b[i], _vector_b[i], _dl_p_key); // b^\prime
    }

    prove_compute_proof_inner_product(c_L, c_R, L, R, _bit_len, _temp, _temp2, _random_xx, ++k); // recursively
}

void Prover::prove_compute_proof_range_proof(int witness, int is_output) {
    // initialize variables
    mpz_t _gamma; mpz_init(_gamma);
    mpz_t _witness; mpz_init(_witness);
    mpz_t _temp_var1; mpz_init(_temp_var1);
    mpz_t _temp_var2; mpz_init(_temp_var2);
    mpz_t _temp_var3; mpz_init(_temp_var3);
    //mpz_t _V; mpz_init(_V);
    char *_als = new char[bit_len];
    mpz_t *_al = new mpz_t[bit_len];
    mpz_t *_ar = new mpz_t[bit_len];
    mpz_t _alpha; mpz_init(_alpha);
    //mpz_t _A ; mpz_init(_A);
    mpz_t *_SL = new mpz_t[bit_len ];
    mpz_t *_SR = new mpz_t[bit_len ];
    for (int i = 0; i < bit_len; i++) {
        mpz_init(_SL[i]); mpz_urandomm(_SL[i], _gstate, _dl_p_key);
        mpz_init(_SR[i]); mpz_urandomm(_SR[i], _gstate, _dl_p_key);
        gmp_printf("_SL : %Zd\n", _SL[i]);
        gmp_printf("_SR : %Zd\n", _SR[i]);
        mpz_init(_al[i]);
        mpz_init(_ar[i]);
    }
    mpz_t _rho; mpz_init(_rho);
    mpz_t _random_x; mpz_init(_random_x);
    mpz_t _random_y; mpz_init(_random_y);
    mpz_t _random_z; mpz_init(_random_z);
    mpz_t _tau1; mpz_init(_tau1);
    mpz_t _tau2; mpz_init(_tau2);
    mpz_t _t1; mpz_init(_t1);
    mpz_t _t2; mpz_init(_t2);
   
    // operate variables
    mpz_set_ui(_witness, witness); // witness
    mpz_sub_ui(_temp_var1, _dl_p_key, 1);
    for (int i = 0; i < bit_len; i++) { // al, ar
        int a = (int) witness / pow(2, bit_len - 1 - i);
        if (a == 1 ) {
            mpz_set_ui(_al[bit_len - 1 - i], 1);
            mpz_set_ui(_ar[bit_len - 1 - i], 0);
            witness = witness - pow(2, bit_len - 1 - i);    
       }else{
            mpz_set_ui(_al[bit_len - 1 - i], 0);
            mpz_set(_ar[bit_len - 1 - i], _temp_var1);
            //mpz_sub_ui(_ar[bit_len - 1 - i], _ar[bit_len - 1 - i], 1);
       }
    }
    cout << "_al: " << endl;   
    for (int i = 0; i < bit_len; i++) { // al, ar
        gmp_printf("%Zd " , _al[i]);
    }
    cout << endl;
    cout << "_ar: " << endl;   
    for (int i = 0; i < bit_len; i++) { // al, ar
        gmp_printf("%Zd " , _ar[i]);
    }
    cout << endl;

    mpz_urandomm(_gamma, _gstate, _dl_p_key); // gamma
    gmp_printf("gamma: %Zd\n", _gamma);
    mpz_set_ui(_V, 0);
    Basic::pedersen_commitment_scalar(_dl_g_key, _witness, _dl_h_key, _gamma, _V, _temp_var1); // commitment v, _V
    gmp_printf("V1: %Zd\n", _V);
    mpz_urandomm(_alpha, _gstate, _dl_p_key); // alpha 
    mpz_powm(_A, _dl_h_key, _alpha, _dl_q_key); // h ^ \alpha
    Basic::pedersen_commitment_vector(_vector_g, 0, _al, 0, _vector_h, 0, _ar, 0, _temp_var1, bit_len, _temp_var2); // commitment g^al* h^ar
    mpz_mul(_A, _A, _temp_var1);
    mpz_mod(_A, _A, _dl_q_key);
    gmp_printf("_A _1: %Zd\n", _A); 
    
    mpz_urandomm(_rho, _gstate, _dl_p_key); // sample rho
    mpz_set_ui(_S, 1);
    Basic::pedersen_commitment_vector(_vector_g, 0, _SL, 0, _vector_h, 0, _SR, 0, _S, bit_len,  _temp_var1); // _S
    gmp_printf("V1: %Zd\n", _V);
    gmp_printf("_S: %Zd\n", _S);
    mpz_powm(_temp_var1, _dl_h_key, _rho, _dl_q_key);// h ^rho
    gmp_printf("V1: %Zd\n", _V);
    mpz_mul(_S, _S, _temp_var1);
    gmp_printf("_S: %Zd\n", _S);
    mpz_mod(_S, _S, _dl_q_key); // _S 
    gmp_printf("_S: %Zd\n", _S);
    Basic::generate_random(Basic::get_msg(_A, _S), _random_y);
    Basic::generate_random(Basic::get_msg(_A, _S, _random_y), _random_z);
    mpz_mod(_random_y, _random_y, _dl_p_key); // y \in z_p
    mpz_mod(_random_z, _random_z, _dl_p_key); // z \in z_p
    mpz_urandomm(_tau1, _gstate, _dl_p_key);
    mpz_urandomm(_tau2, _gstate, _dl_p_key);
    get_t1(_al, _ar, _SL, _SR, _random_y, _random_z, _temp_var1, _temp_var2, _temp_var3, _t1);
    get_t2(_SL, _SR, _random_y, _temp_var1, _t2);
    gmp_printf("t1: %Zd\n", _t1);
    gmp_printf("t2: %Zd\n", _t2);
    Basic::pedersen_commitment_scalar(_dl_g_key, _t1, _dl_h_key, _tau1, _T1, _temp_var1);
    Basic::pedersen_commitment_scalar(_dl_g_key, _t2, _dl_h_key, _tau2, _T2, _temp_var1);
    Basic::generate_random(Basic::get_msg(_T1, _T2), _random_x);
    mpz_mod(_random_x, _random_x, _dl_p_key); // x \in z_p
    gmp_printf("_random_x: %Zd\n", _random_x);
    gmp_printf("_random_y: %Zd\n", _random_y);
    gmp_printf("_random_z: %Zd\n", _random_z);
    get_lx(_al, _SL, _random_x, _random_z, _vector_a, _temp_var1);
    get_rx(_ar, _SR, _random_x, _random_y, _random_z, _vector_b, _temp_var1, _temp_var2);
    for (int i = 0; i < bit_len; i ++ ) {
        gmp_printf("_vector a: %Zd\n", _vector_a[i]);
        gmp_printf("_vector b: %Zd\n", _vector_b[i]);
    }
    mpz_set_ui(_hat_t, 0);
    Basic::inner_product(_vector_a, 0, _vector_b, 0, _hat_t, bit_len, _temp_var1);
    get_tau_x(_tau1, _tau2, _random_x, _random_z, _gamma, _tau_x, _temp_var1);
    get_mu(_alpha, _rho, _random_x, _mu);


    mpz_set(_test_tau1, _tau1);
    mpz_set(_test_tau2, _tau2);
    mpz_set(_test_t1, _t1);
    mpz_set(_test_t2, _t2);
    mpz_set(_test_x, _random_x);
    mpz_set(_test_y, _random_y);
    mpz_set(_test_z, _random_z);
    mpz_set(_test_gamma, _gamma);

    if (is_output) {
        cout << "cleaned variables" << endl;
        gmp_printf("gamma: %Zd\n", _gamma);
        gmp_printf("witness: %Zd\n", _witness);
        gmp_printf("alpha: %Zd\n", _alpha);
        gmp_printf("rho: %Zd\n", _rho);
    }
    mpz_clear(_gamma);
    mpz_clear(_witness);
    mpz_clear(_temp_var1);
    mpz_clear(_temp_var2);
    mpz_clear(_temp_var3);
    delete _als;
    mpz_clear(_alpha);

    for (int i = 0; i < bit_len; i++) {
        mpz_clear(_SL[i]); 
        mpz_clear(_SR[i]); 
        mpz_clear(_al[i]);
        mpz_clear(_ar[i]);
    }
    mpz_clear(_rho);
    mpz_clear(_random_x);
    mpz_clear(_random_y);
    mpz_clear(_random_z);
    gmp_printf("t1: %Zd\n", _t1);
    gmp_printf("t2: %Zd\n", _t2);
    gmp_printf("tau1: %Zd\n", _tau1);
    gmp_printf("tau2: %Zd\n", _tau2);

    mpz_clear(_tau1);
    mpz_clear(_tau2);
    mpz_clear(_t1);
    mpz_clear(_t2);

    if (is_output) {
        cout << "variables transferred to verifier" << endl; 
        gmp_printf("_P: %Zd\n", _P);
        gmp_printf("_T1: %Zd\n", _T1);
        gmp_printf("_T2: %Zd\n", _T2);
        gmp_printf("_S: %Zd\n", _S);
        gmp_printf("_A: %Zd\n", _A);
        gmp_printf("_V: %Zd\n", _V);
        gmp_printf("_hat_t: %Zd\n", _hat_t);
        gmp_printf("_mu: %Zd\n", _mu);
        gmp_printf("_tau_x: %Zd\n", _tau_x);
    }

}

