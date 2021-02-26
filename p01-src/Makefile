default: all

all: cp1

# Number of threads to compile the program with.
# 2 is recommended for building on host servers.
num_threads = 2

cp1:
	cargo build --release --bin cp1 -j$(num_threads)
	cp target/release/cp1 ./
	chmod a+x cp1

cp3:
	cargo build --release --bin cp3 -j$(num_threads)
	cp target/release/cp3 ./
	chmod a+x cp3

clean:
	rm -f cp1