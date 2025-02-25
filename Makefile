all: rust cpp

rust:
	cargo build

cpp: src/simd_example.cpp
	clang++ -framework Accelerate -o simd_example src/simd_example.cpp

clean:
	cargo clean
	rm -f simd_example 