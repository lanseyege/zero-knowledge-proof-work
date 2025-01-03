#include <cryptopp/cryptlib.h>
#include <cryptopp/sha.h>
#include <cryptopp/hex.h>
#include <cryptopp/filters.h>
#include <cryptopp/files.h>

#include <iostream>

int main () {
    using namespace CryptoPP;
    using namespace std;
    //HexEncoder encoder(new FileSink(std::cout));

    string msg = "Yoda said, Do or do not. There is no try.";
    string digest;
    SHA256 hash;
    StringSource(msg, true, new HashFilter(hash, new HexEncoder(new StringSink(digest))));
    //hash.Update((const byte*)msg.data(), msg.size());
    //digest.resize(hash.DigestSize());
    //hash.Final((byte*)&digest[0]);
    //StringSource s2(digest);
    std::cout << "Digest: " ;
    cout << digest << endl;
    //StringSource(digest, true, new Redirector(encoder));
    std::cout <<std::endl;
}
