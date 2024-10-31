build:
	mkdir -p lua/
	cargo build --release
	cp target/release/libsubstitute.so lua/substitute.so
