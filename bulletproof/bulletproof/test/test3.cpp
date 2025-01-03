#include "../bullet.h"
#include <math.h>

int main () {
    int q_length = 300 ; // 64;
    int witness = 44; 
    int bit_len = 8;
    cout << "0" << endl;   
    BulletProof bulletproof;
    cout << "1" << endl;   
    bulletproof.setup(bit_len, witness);
    cout << "2" << endl;   
    bulletproof.keygen(q_length, 1);
    cout << "3" << endl;   
    bulletproof.run_protocal_range_proof();

}
