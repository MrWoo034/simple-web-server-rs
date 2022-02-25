build:
	cargo build --release
run: build
	./target/release/simple-web-server
clean:
	rm -r -f ./target