#include "basic.h"

Basic::Basic() {
    mpz_init(_dl_p_key);
    mpz_init(_dl_q_key);
    mpz_init(_dl_g_key);
    mpz_init(_dl_h_key);

    mpz_init(_P);
    mpz_init(_u);
    mpz_init(_c);

    //prime_test_reps = 40;
    _time = clock();
    gmp_randinit_default(_gstate);
    //gmp_randseed_ui(_gstate, _time);
    gmp_randseed_ui(_gstate, 10);
    
}

Basic::~Basic() {
    mpz_clear(_dl_p_key);
    mpz_clear(_dl_q_key);
    mpz_clear(_dl_g_key);
    mpz_clear(_dl_h_key);

    mpz_clear(_P);
    mpz_clear(_u);
    mpz_clear(_c);

    for (int i = 0; i < bit_len; i++) {
        mpz_clear(_vector_g[i]);
        mpz_clear(_vector_h[i]);
    }
}

void Basic::set_public(mpz_t m_dl_p_key, mpz_t m_dl_q_key, mpz_t m_dl_g_key, mpz_t m_dl_h_key, mpz_t m_u, mpz_t *m_vector_g, mpz_t *m_vector_h, int _bit_len) {
    bit_len = _bit_len; _logn = (int)log2(bit_len);
    _vector_g = new mpz_t[bit_len];
    _vector_h = new mpz_t[bit_len];
    for (int i = 0; i < bit_len; i++) {
        mpz_init(_vector_g[i]);
        mpz_set(_vector_g[i], m_vector_g[i]);
        mpz_init(_vector_h[i]);
        mpz_set(_vector_h[i], m_vector_h[i]);
    }

    mpz_set(_dl_p_key , m_dl_p_key);
    mpz_set(_dl_q_key , m_dl_q_key);
    mpz_set(_dl_g_key , m_dl_g_key);
    mpz_set(_dl_h_key , m_dl_h_key);
    mpz_set(_u, m_u);
}

void Basic::generate_random(string msg, mpz_t _random) {
    string digest;
    StringSource(msg, true, new HashFilter(sha256, new HexEncoder(new StringSink(digest))));
    mpz_set_str(_random, digest.c_str(), 16);
}

void Basic::pedersen_commitment_scalar(mpz_t _base1, mpz_t _inx1, mpz_t _base2, mpz_t _inx2, mpz_t _result, mpz_t _temp) {
    mpz_powm(_result, _base1, _inx1, _dl_q_key);
    mpz_powm(_temp, _base2, _inx2, _dl_q_key);
    mpz_mul(_result, _result, _temp);
    mpz_mod(_result, _result, _dl_q_key);
}

void Basic::pedersen_commitment_vector(mpz_t *_base1, int _start_b1, mpz_t *_inx1, int _start_inx1, mpz_t *_base2, int _start_b2, mpz_t *_inx2, int _start_inx2, mpz_t _result, int _len, mpz_t _temp) {
    //mpz_init(_vector_a[0]); mpz_set_ui(_vector_a[0], 1001);
    //gmp_printf("_vector_a 0: %Zd\n", _vector_a[0]);
    if (mpz_cmp_ui(_result , 1) != 0) mpz_set_ui(_result, 1);
    for (int i = 0; i < _len; i++) {
        mpz_powm(_temp, _base1[i + _start_b1] , _inx1[i + _start_inx1], _dl_q_key);
        mpz_mul(_result, _result, _temp);
    }
    for (int i = 0; i < _len; i++) {
        mpz_powm(_temp, _base2[i + _start_b2] , _inx2[i + _start_inx2], _dl_q_key);
        mpz_mul(_result, _result, _temp);
    }

    mpz_mod(_result, _result, _dl_q_key);
}

void Basic::hadamard_product(mpz_t *_left , int _left_start, mpz_t *_right, int _right_start, mpz_t *_result, int _len, mpz_t _pkey) {
    for (int i = 0; i < _len; i++) {
        mpz_mul(_result[i], _left[i + _left_start], _right[i + _right_start]);
        mpz_mod(_result[i], _result[i], _pkey);
    }
}

void Basic::inner_product(mpz_t *_left, int left_start, mpz_t *_right, int right_start, mpz_t _result, int _len, mpz_t _temp) {
    mpz_set_ui(_result, 0);
    for (int i = 0; i < _len; i++) {
        mpz_mul(_temp, _left[i + left_start], _right[i + right_start]);
        mpz_add(_result, _result, _temp);
    }
    mpz_mod(_result, _result, _dl_p_key);
}

void Basic::pow_scalar(mpz_t *_base, int _start, mpz_t index, int _len) {
    for (int i = 0; i < _len; i ++) {
        mpz_powm(_base[i + _start], _base[i + _start], index, _dl_q_key);
    }
}

void Basic::mul_scalar(mpz_t *_base, int _start, mpz_t _scalar, int _len) {
    for (int i = 0; i < _len; i ++) {
        mpz_mul(_base[i + _start] , _base[i + _start], _scalar);
        mpz_mod(_base[i + _start], _base[i + _start], _dl_q_key);
    }
}

string Basic::get_msg(mpz_t _t1, mpz_t _t2) {
    string msg = "";
    char _temp[1000] ; //= new char[bit_len];
    mpz_get_str(_temp, 16, _t1);
    msg += string(_temp) ;
    mpz_get_str(_temp, 16, _t2);
    msg += string(_temp);
    //delete _temp;
    return msg;
}

string Basic::get_msg(mpz_t _t1, mpz_t _t2, mpz_t _t3) {
    string msg = "";
    char _temp[1000] ; // = new char[bit_len];
    mpz_get_str(_temp, 16, _t1);
    msg += string(_temp) ;
    mpz_get_str(_temp, 16, _t2);
    msg += string(_temp);
    mpz_get_str(_temp, 16, _t3);
    msg += string(_temp);
    //delete _temp;
    return msg;

}

string Basic::get_msg_phug() {
    string msg = ""; 
    char _temp[1000]; // = new char[1000];
    for (int i = 0; i < bit_len; i++ ) {
        mpz_get_str(_temp , 16, _vector_g[i]);
        msg += string(_temp);
    }
    for (int i = 0; i < bit_len; i++ ) {
        mpz_get_str(_temp , 16, _vector_h[i]);
        msg += string(_temp);
    }
    mpz_get_str(_temp, 16, _P);
    gmp_printf("basic: %Zd\n", _P);
    msg += string(_temp);
    mpz_get_str(_temp , 16, _u);
    msg += string(_temp);
    //delete _temp;
    return msg;
}

