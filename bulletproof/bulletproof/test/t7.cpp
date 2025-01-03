#include <iostream>
#include <gmpxx.h>

using namespace std;

int main () {
    mpz_t p; mpz_init(p);
    mpz_t g; mpz_init(g);
    mpz_t h; mpz_init(h);
    mpz_t v; mpz_init(v);
    mpz_t m; mpz_init(m);

    string _p = "32039736547859699279";
    string _g = "10869219268717386479";
    string _h = "31322976265318203043";
    string _v = "45";
    string _m = "4105867546894256506";

    mpz_set_str(p, _p.c_str(), 10);
    mpz_set_str(g, _g.c_str(), 10);
    mpz_set_str(h, _h.c_str(), 10);
    mpz_set_str(v, _v.c_str(), 10);
    mpz_set_str(m, _m.c_str(), 10);

    mpz_t _temp1; mpz_init(_temp1);
    mpz_t _temp2; mpz_init(_temp2);
    mpz_powm(_temp1, g, v, p);
    mpz_powm(_temp2, h, m, p);
    mpz_mul(_temp1, _temp1, _temp2);
    mpz_mod(_temp1, _temp1, p);
    gmp_printf(" %Zd\n", _temp1);

    mpz_t _z; mpz_init(_z);
    mpz_t _x; mpz_init(_x);
    mpz_t _t1; mpz_init(_t1);
    mpz_t _t2; mpz_init(_t2);
    mpz_t _tau1; mpz_init(_tau1);
    mpz_t _tau2; mpz_init(_tau2);
    mpz_t _gamma; mpz_init(_gamma);
    mpz_t _p1; mpz_init(_p1);
    mpz_t _delta; mpz_init(_delta);

    mpz_set_str(_z, "105165185173229613961125340572191529525026907675571218304499113121052159389925", 10);
    mpz_set_str(_x, "88129672684986824872051029387979277712946309644882288143932644057298403009885", 10);
    mpz_set_str(_t1, "22173791813426848425", 10);
    mpz_set_str(_t2, "11530201907524450174", 10);
    mpz_set_str(_gamma, "16439763794640544783", 10);
    mpz_set_str(_delta, "14427224011103897018", 10);
    mpz_set_str(_p1, "30230238713027151419", 10);
    mpz_set_str(_tau1, "3479038662069152906", 10);
    mpz_set_str(_tau2, "28134540436397284266", 10);

    mpz_mul(_temp1, _z, _z);
    mpz_mod(_temp1, _temp1, _p1);
    mpz_mul(_temp1, v, _temp1);
    mpz_add(_temp1, _temp1, _delta);
    mpz_mul(_temp2, _t1, _x);
    mpz_add(_temp1, _temp1, _temp2);
    mpz_mul(_temp2, _x, _x);
    mpz_mul(_temp2, _t2, _temp2);
    mpz_add(_temp1, _temp1, _temp2);
    mpz_mod(_temp1, _temp1, _p1);
    gmp_printf("hat t: %Zd\n", _temp1);


    mpz_mul(_temp1, _z, _z);
    mpz_mul(_temp1, _temp1, _gamma);
    mpz_mul(_temp2, _tau1, _x);
    mpz_add(_temp1, _temp1, _temp2);
    mpz_mul(_temp2, _x, _x);
    mpz_mul(_temp2, _temp2, _tau2);
    mpz_add(_temp1, _temp1, _temp2);
    mpz_mod(_temp1, _temp1, _p1);
    gmp_printf("tau x: %Zd\n", _temp1);


}
