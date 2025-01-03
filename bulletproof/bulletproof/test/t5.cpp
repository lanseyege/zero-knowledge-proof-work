#include "gmpxx.h"
#include <iostream>
#include <math.h>
#include <stdio.h>

using namespace std;

int main () {
    string s1 = "";
    mpz_t a; mpz_init(a);
    mpz_set_ui(a , 100001);
    char *temp = new char[1000];
    mpz_get_str(temp, 16, a);
    s1 = s1 + string(temp);
    cout << temp << endl;
    mpz_get_str(temp, 10, a);
    s1 = s1 + string(temp);
    cout << temp << endl;

    cout << s1 << endl;
    int *_te = new int[1000];
    //mpz_get_str(_te, 2, a);
    cout << temp << endl;
    for (int i = 0; i < 10; i ++) {
        cout << temp[i] - '0'<< endl;
    }
    delete temp;
    int n = 0;
    n = (int)log (64);
    cout << n << endl;
}
