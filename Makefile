build:
	mkdir -p lua/
	cargo build --release
	cp target/release/libtext_case.so lua/text_case.so
