#include "../bullet.h"

int main() {
    BulletProof bullet ;
    int q_length = 100;
    int is_output = 1;
    bullet.setup();
    bullet.keygen(q_length, is_output);
    bullet.pedersen_commitment();
} 
