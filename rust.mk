CLEAN_RUST := clean-rust

rust: rust-target/release/rust
	ln -f $< $@

rust-target/release/rust: rust.rs
	cargo build --release

clean-rust:
	$(RM) -r rust rust-target
