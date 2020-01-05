#ifndef BIGNUM_INCLUDED
#define BIGNUM_INCLUDED
#include <cmath>
#include <cstdint>
#include <numeric>
#include <vector>
using BigNum = std::vector<uint8_t>;
uint8_t getn(const size_t number, const uint8_t n);
BigNum bignum_mult(BigNum sum, const uint8_t multer);
int sum_bignum_digits(const BigNum& number);
BigNum split_to_bignum(size_t number);
#endif
