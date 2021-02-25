
doc:
	RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo +nightly doc --bins --no-deps --release
	rm -rf ./docs
	cp -r ./target/doc ./docs
