
doc:
	rm -rf ./docs
	cargo doc --bins --no-deps --release
	cp -r ./target/doc ./docs
