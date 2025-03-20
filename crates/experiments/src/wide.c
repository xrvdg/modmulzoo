#include <arm_neon.h>
#include <stdio.h>

void print_uint32x2(uint32x2_t v, const char *name)
{
    uint32_t arr[2];
    vst1_u32(arr, v);
    printf("%s: \n", name);
    printf("  dec: [%u, %u]\n", arr[0], arr[1]);
    printf("  hex: [0x%08x, 0x%08x]\n", arr[0], arr[1]);
}

void print_uint32x4(uint32x4_t v, const char *name)
{
    uint32_t arr[4];
    vst1q_u32(arr, v);
    printf("%s: \n", name);
    printf("  dec: [%u, %u, %u, %u]\n", arr[0], arr[1], arr[2], arr[3]);
    printf("  hex: [0x%08x, 0x%08x, 0x%08x, 0x%08x]\n", arr[0], arr[1], arr[2], arr[3]);
}

void print_uint64x2(uint64x2_t v, const char *name)
{
    uint64_t arr[2];
    vst1q_u64(arr, v);
    printf("%s: \n", name);
    printf("  dec: [%lu, %lu]\n", arr[0], arr[1]);
    printf("  hex: [0x%016lx, 0x%016lx]\n", arr[0], arr[1]);
}

int main()
{
    // First demonstration: Widening addition
    printf("=== Widening Addition Demo ===\n");
    uint32x4_t a = {10, 20, 30, 40};
    uint64x2_t b = {100, 200};

    printf("Original vectors:\n");
    print_uint32x4(a, "a (uint32x4_t)");
    print_uint64x2(b, "b (uint64x2_t)");

    uint64x2_t result1 = vaddw_u32(b, vget_low_u32(a));
    printf("\nvaddw_u32 result:\n");
    printf("Operation: b + low_half(a)\n");
    print_uint64x2(result1, "result1");

    uint64x2_t result2 = vaddw_high_u32(b, a);
    printf("\nvaddw_high_u32 result:\n");
    printf("Operation: b + high_half(a)\n");
    print_uint64x2(result2, "result2");

    // Second demonstration: Rounding narrowing
    printf("\n=== Rounding Narrowing Demo ===\n");

    // Using values that will demonstrate rounding behavior
    uint64x2_t x = {0x7000000000000000ULL, 0x7100000000000000ULL}; // Large values to show rounding
    uint64x2_t y = {0x7200000000000000ULL, 0x7300000000000000ULL};

    printf("Large number vectors:\n");
    print_uint64x2(x, "x (uint64x2_t)");
    print_uint64x2(y, "y (uint64x2_t)");

    // vraddhn_u64: Gets lower half result, rounding narrowing
    uint32x2_t narrow1 = vraddhn_u64(x, y);
    printf("\nvraddhn_u64 result (first half):\n");
    print_uint32x2(narrow1, "narrow1");

    // Create another pair of vectors for the high half
    uint64x2_t z = {0x7400000000000000ULL, 0x7500000000000000ULL};
    uint64x2_t w = {0x7600000000000000ULL, 0x7700000000000000ULL};

    printf("\nSecond pair of vectors:\n");
    print_uint64x2(z, "z (uint64x2_t)");
    print_uint64x2(w, "w (uint64x2_t)");

    // This creates a full uint32x4_t by combining narrow1 with the new narrowed values
    uint32x4_t narrow_full = vraddhn_high_u64(narrow1, z, w);
    printf("\nvraddhn_high_u64 result (combined):\n");
    print_uint32x4(narrow_full, "narrow_full");

    return 0;
}