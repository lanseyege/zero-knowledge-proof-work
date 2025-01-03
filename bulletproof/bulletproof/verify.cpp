#include "basic.h"

Verifier::Verifier() {
    mpz_init(_test_delta_yz);
}

Verifier::~Verifier() {
    mpz_clear(_test_delta_yz);
}

int Verifier::verify_compute_proof_range_proof(mpz_t _tau_x, mpz_t _mu, mpz_t _hat_t, mpz_t _V, mpz_t _A, mpz_t _S, mpz_t _T1, mpz_t _T2) {
    mpz_t _random_x; mpz_init(_random_x); 
    mpz_t _random_y; mpz_init(_random_y);
    mpz_t _random_z; mpz_init(_random_z);
    mpz_t _temp_z; mpz_init(_temp_z);
    mpz_t _temp_var1; mpz_init(_temp_var1);
    mpz_t _temp_var2; mpz_init(_temp_var2);
    mpz_t _temp_var3; mpz_init(_temp_var3);
    mpz_t _temp_var4; mpz_init(_temp_var4);
    mpz_t _temp_var5; mpz_init(_temp_var5);
    mpz_t _check_tl; mpz_init(_check_tl);
    mpz_t _check_tr; mpz_init(_check_tr);
    mpz_t _delta_yz; mpz_init(_delta_yz);

    mpz_t _PL; mpz_init(_PL);
    mpz_t _PR; mpz_init(_PR);

    Basic::generate_random(Basic::get_msg(_T1, _T2), _random_x);
    Basic::generate_random(Basic::get_msg(_A, _S), _random_y);
    Basic::generate_random(Basic::get_msg(_A, _S, _random_y), _random_z);
    mpz_mod(_random_x, _random_x, _dl_p_key);
    mpz_mod(_random_y, _random_y, _dl_p_key);
    mpz_mod(_random_z, _random_z, _dl_p_key);
    cout << "verify " << endl;
    gmp_printf("_random_x: %Zd\n", _random_x);
    gmp_printf("_random_y: %Zd\n", _random_y);
    gmp_printf("_random_z: %Zd\n", _random_z);
    mpz_set_ui(_temp_var1, 1);
    for (int i = 1 ; i < bit_len ; i ++) {
        mpz_mul(_temp_var1, _temp_var1, _random_y);
        mpz_invert( _temp_var2, _temp_var1, _dl_p_key);
        mpz_powm(_vector_h[i], _vector_h[i], _temp_var2, _dl_q_key );
        gmp_printf("vector h p: %Zd\n", _vector_h[i]);
    }
    Basic::pedersen_commitment_scalar(_dl_g_key, _hat_t, _dl_h_key, _tau_x, _check_tl, _temp_var2); // g^\hat{t} * h^\tau_x
    mpz_set_ui(_temp_var1, 1);
    mpz_set_ui(_temp_var2, 1);
    for (int i = 1; i < bit_len; i++) {
        mpz_mul(_temp_var1, _temp_var1, _random_y);
        mpz_mod(_temp_var1, _temp_var1, _dl_p_key);
        mpz_add(_temp_var2, _temp_var2, _temp_var1);
        //mpz_mod(_temp_var2, _temp_var2, _dl_p2_key);
    } 
    mpz_mod(_temp_var2, _temp_var2, _dl_p_key);// y ^ n
    mpz_mul(_temp_var1, _random_z, _random_z); // z^2
    mpz_sub(_temp_var1, _random_z, _temp_var1); //(z - z^2)
    mpz_mul(_delta_yz , _temp_var1, _temp_var2); // (z - z^2)*y^n
    //mpz_mod(_delta_yz, _delta_yz, _dl_p_key); // delta_yz \in Z_p

    mpz_mul(_temp_var1, _random_z, _random_z); // z^2
    mpz_mul(_temp_var1, _temp_var1, _random_z); // z^3
    mpz_set_ui(_temp_var2, (int) pow(2, bit_len) - 1); // <1^n, 2^n>
    mpz_mul(_temp_var1, _temp_var1, _temp_var2); // z^3 * 2^n
    mpz_sub(_delta_yz, _delta_yz, _temp_var1); // . - . 
    mpz_mod(_delta_yz, _delta_yz, _dl_p_key); // delta_yz \in Z_p
    gmp_printf("delta: %Zd\n", _delta_yz);
    mpz_set(_test_delta_yz, _delta_yz);

    mpz_powm(_temp_var1, _dl_g_key, _delta_yz, _dl_q_key); // g^delta
    gmp_printf("g^delta: %Zd\n", _temp_var1);
    mpz_mul(_temp_var2, _random_z, _random_z); // z^2
    //mpz_mod(_temp_var2, _temp_var2, _dl_p_key);
    mpz_powm(_temp_var2, _V, _temp_var2, _dl_q_key); // V^{z^2}
    gmp_printf("V^z2: %Zd\n", _temp_var2);
    
    mpz_mul(_check_tr, _temp_var1, _temp_var2); // 
    mpz_powm(_temp_var1, _T1, _random_x, _dl_q_key); // T_1^x
    gmp_printf("T1^x: %Zd\n", _temp_var1);

    mpz_mul(_check_tr, _check_tr, _temp_var1);
    mpz_mul(_temp_var2, _random_x, _random_x); // x^2
    //mpz_mod(_temp_var2, _temp_var2, _dl_p_key); // x^2 mod p
    mpz_powm(_temp_var2 , _T2, _temp_var2, _dl_q_key); // T_2^{x^2}
    gmp_printf("T2^x2: %Zd\n", _temp_var2);

    mpz_mul(_check_tr, _check_tr, _temp_var2); // 
    mpz_mod(_check_tr, _check_tr, _dl_q_key); // 
    if (mpz_cmp(_check_tl, _check_tr) != 0) { // check 1
        cout << endl;
        gmp_printf("_check_tl: %Zd\n", _check_tl);
        gmp_printf("_check_tr: %Zd\n", _check_tr);
        cout << "_hat_t != t(x) , return !!" << endl;    
        cout << endl;
        return 0;
    } else {
        cout << endl;
        gmp_printf("_check_tl: %Zd\n", _check_tl);
        gmp_printf("_check_tr: %Zd\n", _check_tr);
        cout << "_hat_t = t(x) , continue !!" << endl;    
        cout << endl;
    } 
    // calculating P 
    mpz_powm(_temp_var1, _S, _random_x, _dl_q_key); // S^x
    mpz_mul(_temp_var1, _A, _temp_var1); // A * S^x
    mpz_set_ui(_PL, 1);
    for (int i = 0; i < bit_len; i ++ ) {
        mpz_powm(_temp_var2 , _vector_g[i], _random_z, _dl_q_key);
        mpz_invert(_temp_var2 , _temp_var2, _dl_q_key);
        gmp_printf("vector g reverse: %Zd\n", _temp_var2);
        mpz_mul(_PL, _PL, _temp_var2);
    }
    mpz_mod(_PL, _PL, _dl_q_key); // g^-z
    mpz_mul(_PL, _PL, _temp_var1); // A * S^x * g^-z 
    mpz_mod(_PL, _PL, _dl_q_key); // 
    mpz_set_ui(_temp_var1, 1); // record y^n
    mpz_set_ui(_temp_var2, 1); // record 2^n
    mpz_mul(_temp_z, _random_z, _random_z); // record z^2
    mpz_set_ui(_temp_var5, 1);
    for (int i = 0; i < bit_len; i ++) {// y^n
        mpz_mul(_temp_var3, _random_z, _temp_var1); // z * y^i
        mpz_mul(_temp_var4, _temp_z, _temp_var2); // z^2 * 2^i
        mpz_add(_temp_var3, _temp_var3, _temp_var4); // . + .

        mpz_powm(_temp_var4, _vector_h[i], _temp_var3, _dl_q_key);
        mpz_mul(_temp_var5 , _temp_var5, _temp_var4); // multiple continuously

        mpz_mul(_temp_var1, _temp_var1, _random_y);
        mpz_mod(_temp_var1, _temp_var1, _dl_p_key);
        mpz_mul_ui(_temp_var2, _temp_var2, 2);
        mpz_mod(_temp_var2, _temp_var2, _dl_p_key);
    }
    mpz_mod(_temp_var5, _temp_var5, _dl_q_key);
    mpz_mul(_PL, _PL, _temp_var5);
    mpz_mod(_PL, _PL, _dl_q_key); // get _PL
    gmp_printf("_PL: %Zd\n", _PL);

    mpz_powm(_temp_var1, _dl_h_key, _mu, _dl_q_key); // h^\mu
    mpz_invert(_temp_var1, _temp_var1, _dl_q_key);  // h^-mu
    mpz_mul(_PL, _PL, _temp_var1);
    mpz_mod(_P, _PL, _dl_q_key); // p*h^-mu
    gmp_printf("_P: %Zd\n", _P);

    //Basic::pedersen_commitment_vector(_vector_g, 0, );
    
    mpz_clear(_random_x);
    mpz_clear(_random_y);
    mpz_clear(_random_z);
    mpz_clear(_temp_z);
    mpz_clear(_temp_var1);
    mpz_clear(_temp_var2);
    mpz_clear(_temp_var3);
    mpz_clear(_temp_var4);
    mpz_clear(_temp_var5);
    mpz_clear(_check_tl);
    mpz_clear(_check_tr);
    mpz_clear(_delta_yz);
    mpz_clear(_PL);
    mpz_clear(_PR);

    return 1;
}

int Verifier::verify_compute_proof_inner_product(mpz_t _vector_g0, mpz_t  _vector_h0, mpz_t _vector_a0, mpz_t _vector_b0, mpz_t _P0, mpz_t m_u, mpz_t *_array_l, mpz_t *_array_r) {
    //mpz_set(_u, m_u);
    gmp_printf("start1 _P: %Zd\n", _P);
    gmp_printf("start1 _u: %Zd\n", _u);
    gmp_printf("start1 _c: %Zd\n", _c);

    int _bit_len = bit_len; 
    mpz_t _random_xx; mpz_init(_random_xx);
    mpz_t _temp_var1; mpz_init(_temp_var1);
    mpz_t _P_prime; mpz_init(_P_prime);
    mpz_t _cc; mpz_init(_cc);
    Basic::generate_random(Basic::get_msg_phug() , _random_xx); // get random x
    cout << "msg hpug: " << Basic::get_msg_phug() << endl;
    gmp_printf("_random xx: %Zd\n", _random_xx);
    mpz_mod(_random_xx, _random_xx, _dl_p_key);
    //mpz_mul(_temp_var1, _random_xx, _hat_t); // x * c
    mpz_mul(_temp_var1, _random_xx, _c); // x * c
    mpz_powm(_temp_var1 , _u, _temp_var1, _dl_q_key); // u^{x * c}
    mpz_mul(_P, _temp_var1, _P);
    mpz_mod(_P, _P, _dl_q_key);
    mpz_powm(_u, _u, _random_xx, _dl_q_key);
    gmp_printf("start2 _P: %Zd\n", _P);
    gmp_printf("start2 _u: %Zd\n", _u);

    for (int i = 0 ; i < _logn; i++) {
        cout << " i : " << i << endl;
        _bit_len = (int) _bit_len / 2;
        Basic::generate_random(Basic::get_msg(_array_l[i], _array_r[i]), _random_xx); // get random x
        mpz_mod(_random_xx, _random_xx, _dl_p_key);
        gmp_printf("_array_l: %Zd\n", _array_l[i]);
        gmp_printf("_array_r: %Zd\n", _array_r[i]);
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
        mpz_invert(_temp_var1 , _random_xx, _dl_p_key);
        Basic::pow_scalar(_vector_g, 0, _temp_var1, _bit_len);
        Basic::pow_scalar(_vector_g, _bit_len, _random_xx, _bit_len);
        Basic::hadamard_product(_vector_g, 0, _vector_g, _bit_len, _vector_g, _bit_len, _dl_q_key ); // get g^\prime
        // step 30
        Basic::pow_scalar(_vector_h, 0, _random_xx, _bit_len);
        Basic::pow_scalar(_vector_h, _bit_len, _temp_var1, _bit_len);
        Basic::hadamard_product(_vector_h, 0, _vector_h, _bit_len, _vector_h, _bit_len, _dl_q_key); // get h^\prime
        // step 31 
        mpz_mul(_temp_var1, _random_xx, _random_xx);
        //mpz_mod(_temp, _temp, _dl_p_key);
        mpz_powm(_temp_var1, _array_l[i], _temp_var1, _dl_q_key);
        mpz_mul(_P, _temp_var1, _P);
        mpz_mod(_P, _P, _dl_q_key);
        mpz_mul(_temp_var1, _random_xx, _random_xx);
        mpz_invert(_temp_var1 , _temp_var1, _dl_p_key);
        mpz_powm(_temp_var1, _array_r[i], _temp_var1, _dl_q_key);
        mpz_mul(_P, _P, _temp_var1);
        mpz_mod(_P, _P, _dl_q_key); // get P^\prime 
        gmp_printf("_P in : %Zd\n", _P);
    }
    //gmp_printf("verifier P0: %Zd\n", _P);
    Basic::pedersen_commitment_scalar(_vector_g0, _vector_a0, _vector_h0, _vector_b0, _P_prime, _temp_var1); // g^a*h^b
    mpz_mul(_cc, _vector_a0, _vector_b0); // c = a * b
    mpz_powm(_temp_var1, _u, _cc, _dl_q_key); // u^c
    mpz_mul(_P_prime, _P_prime, _temp_var1);
    mpz_mod(_P_prime, _P_prime, _dl_q_key);
    gmp_printf("_P:       %Zd\n", _P);
    gmp_printf("_P_prime: %Zd\n", _P_prime);

    mpz_clear(_random_xx);
    mpz_clear(_temp_var1);
    mpz_clear(_cc);

    int res = -1;
    if (mpz_cmp(_P, _P_prime) == 0) {
        res = 1 ;
    }else {
        res = 0 ;
    }
    mpz_clear(_P_prime);

    return res; 
}


