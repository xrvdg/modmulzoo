#include <simd/simd.h>
#include <iostream>
#include <iomanip>

int main()
{

    // Create SIMD vectors of 8 doubles
    simd_double8 vector1 = {1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0};
    simd_double8 vector2 = {2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0};

    // Perform SIMD operations
    // Does NOT use AMX instructions. It just repeats fadd.2d four times
    simd_double8 sum = vector1 + vector2;     // Vector addition
    simd_double8 product = vector1 * vector2; // Vector multiplication

    // Print results
    std::cout << "Vector 1: ";
    for (int i = 0; i < 8; i++)
    {
        std::cout << std::fixed << std::setprecision(1) << vector1[i] << " ";
    }
    std::cout << "\n";

    std::cout << "Vector 2: ";
    for (int i = 0; i < 8; i++)
    {
        std::cout << std::fixed << std::setprecision(1) << vector2[i] << " ";
    }
    std::cout << "\n";

    std::cout << "Sum: ";
    for (int i = 0; i < 8; i++)
    {
        std::cout << std::fixed << std::setprecision(1) << sum[i] << " ";
    }
    std::cout << "\n";

    std::cout << "Product: ";
    for (int i = 0; i < 8; i++)
    {
        std::cout << std::fixed << std::setprecision(1) << product[i] << " ";
    }
    std::cout << "\n";

    // Additional SIMD operations
    simd_double8 min = simd_min(vector1, vector2); // Element-wise minimum
    simd_double8 max = simd_max(vector1, vector2); // Element-wise maximum
    simd_double8 rsqrt = simd_rsqrt(vector1);      // Element-wise reciprocal square root

    std::cout << "\nAdditional operations:\n";

    std::cout << "Min: ";
    for (int i = 0; i < 8; i++)
    {
        std::cout << std::fixed << std::setprecision(1) << min[i] << " ";
    }
    std::cout << "\n";

    std::cout << "Max: ";
    for (int i = 0; i < 8; i++)
    {
        std::cout << std::fixed << std::setprecision(1) << max[i] << " ";
    }
    std::cout << "\n";

    std::cout << "Reciprocal sqrt of vector1: ";
    for (int i = 0; i < 8; i++)
    {
        std::cout << std::fixed << std::setprecision(3) << rsqrt[i] << " ";
    }
    std::cout << "\n";

    return 0;
}