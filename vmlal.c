#include <stdio.h>
#include <arm_neon.h>

void print_uint32x2(const char *label, uint32x2_t vec)
{
    uint32_t values[2];
    vst1_u32(values, vec);
    printf("%s: [%u, %u]\n", label, values[0], values[1]);
}

void print_uint32x4(const char *label, uint32x4_t vec)
{
    uint32_t values[4];
    vst1q_u32(values, vec);
    printf("%s: [%u, %u, %u, %u]\n", label, values[0], values[1], values[2], values[3]);
}

void print_uint64x2(const char *label, uint64x2_t vec)
{
    uint64_t values[2];
    vst1q_u64(values, vec);
    printf("%s: [%lu, %lu]\n", label, values[0], values[1]);
}

int main()
{
    // Initialize test vectors with volatile to prevent optimization
    volatile uint32_t a_data[4] = {1, 2, 3, 4}; // 4 32-bit values
    volatile uint32_t b_data[4] = {5, 6, 7, 8}; // 4 32-bit values
    uint64_t acc_data[2] = {100, 200};          // 2 64-bit accumulators

    // Load vectors from copies
    uint32x4_t ab = vld1q_u32(a_data);    // Load all 4 values
    uint32x4_t bb = vld1q_u32(b_data);    // Load all 4 values
    uint64x2_t acc = vld1q_u64(acc_data); // Load accumulators

    // Extract low parts (first two elements)
    uint32x2_t a_low = vget_low_u32(ab); // Gets elements [0,1]
    uint32x2_t b_low = vget_low_u32(bb); // Gets elements [0,1]

    // Demonstrate vmlal_u32 (uses low parts)
    uint64x2_t result_low = vmlal_u32(acc, a_low, b_low);

    // Reset accumulator
    acc = vld1q_u64(acc_data);

    // Demonstrate vmlal_high_u32 (uses high parts)
    uint64x2_t result_high = vmlal_high_u32(acc, ab, bb);

    printf("Initial values:\n");
    print_uint32x4("Full a vector", ab);
    print_uint32x4("Full b vector", bb);
    print_uint64x2("Initial acc", acc);
    printf("\n");

    printf("vmlal_u32 operation (using low parts):\n");
    print_uint32x2("a_low", a_low);
    print_uint32x2("b_low", b_low);
    print_uint64x2("result_low", result_low);
    printf("This computes: acc + (a_low * b_low) element-wise\n");
    printf("i.e., [100 + (1 * 5), 200 + (2 * 6)]\n\n");
    // Extract high parts (last two elements)
    uint32x2_t a_high = vget_high_u32(ab); // Gets elements [2,3]
    uint32x2_t b_high = vget_high_u32(bb); // Gets elements [2,3]

    printf("vmlal_high_u32 operation (using high parts):\n");
    print_uint32x2("a_high", a_high);
    print_uint32x2("b_high", b_high);
    print_uint64x2("result_high", result_high);
    printf("This computes: acc + (a_high * b_high) element-wise\n");
    printf("i.e., [100 + (3 * 7), 200 + (4 * 8)]\n");

    return 0;
}