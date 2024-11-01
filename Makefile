build: clean
	cargo build --release
	cp target/release/libsubstitute.so lua/substitute/utils.so

substitute:
	mkdir -p lua/substitute/

clean: substitute
	cargo clean
	rm -rf lua/**.so
