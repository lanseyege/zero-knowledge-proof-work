#include <iostream>
#include <stdio.h>
#include <math.h>
#include <gmpxx.h>
#include <time.h>
#include <stdlib.h>

using namespace std;

int main() {
    mpz_t p; mpz_init(p);
    mpz_t q; mpz_init(q);
    mpz_t g; mpz_init(g);
    mpz_t h; mpz_init(h);
    mpz_t temp; mpz_init(temp);
    int p_length = 100;
    int prime_test = 20;
    gmp_randstate_t _gstate;
    gmp_randinit_default(_gstate);
    gmp_randseed_ui(_gstate, 10);

    mpz_urandomb(p, _gstate, p_length);
    mpz_nextprime(p, p);
    mpz_mul_ui(q, p, 2);
    mpz_add_ui(q, q, 1);

    while (mpz_probab_prime_p(q, prime_test) != 1) {
        mpz_nextprime(p, p);
        mpz_mul_ui(q, p, 2);
        mpz_add_ui(q, q, 1);
    }
    
    mpz_urandomm(g, _gstate, q);
    while (mpz_cmp(g, p) != 0 ) {
        mpz_powm(temp, g, p, q);
        if (mpz_cmp_ui(temp , 1) == 0) {
            break;
        }else{
            mpz_urandomm(g, _gstate, q);
        }
    }
    mpz_urandomm(h, _gstate, q);
    while (mpz_cmp(h, p) != 0 ) {
        mpz_powm(temp, h, p, q);
        if (mpz_cmp_ui(temp , 1) == 0) {
            break;
        }else{
            mpz_urandomm(h, _gstate, q);
        }
    }

    gmp_printf("p %Zd\n", p);
    gmp_printf("q %Zd\n", q);
    gmp_printf("g %Zd\n", g);
    gmp_printf("h %Zd\n", h);
    //gmp_printf()

    mpz_clear(p);
    mpz_clear(q);
    mpz_clear(g);
    mpz_clear(h);
}
