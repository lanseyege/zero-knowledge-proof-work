#include <iostream>
#include <stdio.h>
#include <gmpxx.h>

using namespace std;

int main() {
    cout <<"" ;
    gmp_randstate_t _gstate;
    clock_t _time;
    mpz_t _dl_p_key;
    mpz_t _dl_q_key;
    mpz_t _dl_g1_key;
    mpz_t _dl_g2_key;
    mpz_t _temp_p;
    mpz_t _temp_i;
    mpz_t _temp_h;

    _time = clock();
    gmp_randinit_default(_gstate);
    gmp_randseed_ui(_gstate, _time);

    mpz_init(_dl_p_key);
    mpz_init(_dl_q_key);
    mpz_init(_dl_g1_key);
    mpz_init(_dl_g2_key);
    mpz_init(_temp_p);
    mpz_init(_temp_i);
    mpz_init(_temp_h);

    int q_length = 512;
    mpz_urandomb(_dl_q_key, _gstate, q_length);
    mpz_nextprime(_dl_q_key, _dl_q_key);

    int i = 2;
    int prime_test_reps = 40;
    while(true) {
        cout <<"1"<<endl;
        mpz_mul_ui(_dl_p_key, _dl_q_key, i);
        mpz_add_ui(_dl_p_key, _dl_p_key, 1);
        if (mpz_probab_prime_p(_dl_p_key, prime_test_reps) != 0) break;
        i += 1;
    }
    mpz_sub_ui(_temp_p, _dl_p_key, 1);
    mpz_add_ui(_temp_i, _temp_i, i);
    while(true) {
        cout <<"2"<<endl;
        mpz_urandomm(_temp_h, _gstate, _temp_p);
        mpz_add_ui(_temp_h, _temp_h, 1);
        mpz_powm(_dl_g1_key, _temp_h, _temp_i, _dl_p_key);
        if (mpz_cmp_ui(_dl_g1_key, 1) != 0) break;
    }
    while(true) {
        cout <<"3"<<endl;
        mpz_urandomm(_temp_h, _gstate, _temp_p);
        mpz_add_ui(_temp_h, _temp_h, 1);
        mpz_powm(_dl_g2_key, _temp_h, _temp_i, _dl_p_key);
        if (mpz_cmp_ui(_dl_g2_key, 1) != 0) break;
    }

    gmp_printf("p: %Zd\n", _dl_p_key);
    gmp_printf("q: %Zd\n", _dl_q_key);
    gmp_printf("g1: %Zd\n", _dl_g1_key);
    gmp_printf("g2: %Zd\n", _dl_g2_key);

    mpz_clear(_dl_p_key);
    mpz_clear(_dl_q_key);
    mpz_clear(_dl_g1_key);
    mpz_clear(_dl_g2_key);
    mpz_clear(_temp_p);
    mpz_clear(_temp_i);
    mpz_clear(_temp_h);

}
