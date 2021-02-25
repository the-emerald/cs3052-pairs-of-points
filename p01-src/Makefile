default: all

all: cp1

cp1:
	cargo build --release --bin cp1 -j2
	cp target/release/cp1 ./
	chmod a+x cp1

clean:
	cargo clean
	rm -f cp1