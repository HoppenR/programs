#ifndef BIGNUM_INCLUDED
#define BIGNUM_INCLUDED
#include <cmath>
#include <cstdint>
#include <numeric>
#include <vector>
uint8_t getn(const size_t number, const uint8_t n);
std::vector<uint8_t> bignum_mult(std::vector<uint8_t> sum,
								 const uint8_t multer);
int sum_bignum_digits(const std::vector<uint8_t>& number);
#endif
