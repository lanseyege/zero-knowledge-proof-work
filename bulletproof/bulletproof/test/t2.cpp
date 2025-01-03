#include <iostream>
#include <stdio.h>
#include <gmpxx.h>

using namespace std;

void fun(mpz_t _temp) {
    //mpz_t _temp2;
    
    //gmp_printf("temp: %Zd\n", _temp2);
}

int main() {
    mpz_t _temp; mpz_init(_temp);
    mpz_set_ui(_temp, 10);
    fun(_temp);
    
    int len = 3;
    mpz_t *vec = new mpz_t[len];
    for (int i = 0; i < len; i++) {
        mpz_init(vec[i]);
        //mpz_set_ui(vec[i], i);
        mpz_set(vec[i], _temp);
        gmp_printf("vec %Zd\n", vec[i]);
    }

    for(int i = 0; i < len; i++)
        mpz_clear(vec[i]);
    free(vec);
    return 0;
}
