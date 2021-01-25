
doc:
	rm -rf ./docs
	RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo +nightly doc --bins --no-deps --release
	cp -r ./target/doc ./docs
