build:
	clang -g -O2 -c -target bpf -o minimal.bpf.o src/bpf/minimal.bpf.c
