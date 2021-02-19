
build:
	@cargo build
	@cbindgen --config cbindgen.toml --crate rustc-lib --output rustc-http.h
	@g++ -o target/main main.cpp target/debug/librustc_http.so

run:
	@target/main