build:
	mkdir -p lua/substitute/
	cargo build --release
	cp target/release/libsubstitute.so lua/substitute/utils.so

clean:
	cargo clean
	rm -rf lua/**.so
