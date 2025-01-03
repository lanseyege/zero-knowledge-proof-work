#include <iostream>
#include <stdio.h>
#include <gmpxx.h>
#include <math.h>

using namespace std;

int main() {
    mpz_t temp, temp2, temp3;
    mpz_init(temp);
    mpz_init(temp2);
    mpz_init(temp3);

    mpz_set_ui(temp, 3);
    mpz_set_ui(temp3, 13);
    mpz_invert(temp2, temp, temp3);
    string s1("A02F"); cout << s1.c_str() << endl;
    mpz_set_str(temp2, s1.c_str(), 16);
    gmp_printf("%Zd\n", temp2);
    gmp_printf("%Zx\n", temp2);

    mpz_set_ui(temp, 3);
    char *cls = new char[8];
    
    for (int i = 0; i < 8 ; i++) 
        cls[i] = '0';
    for (int i = 0; i < 8 ; i++)
        cout << cls[i] <<" "; 
    cout << endl;
    int w = 44; int n = 8;
    for (int i = 0; i < n ; i++) {
        int a = (int) w /  pow(2, n- 1 -i);
        if (a == 1) {
            cls[n-1-i] = '1';
            w = w - pow(2, n - 1 - i);
        }
        else cls[n-1-i] = '0';
         
    }
    for (int i = 0; i < 8 ; i++)
        cout << cls[i] <<" "; 

}
