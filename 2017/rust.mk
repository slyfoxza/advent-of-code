.PHONY: clean-rust

rust: rust-target/release/rust
	cp $< $@

rust-target/release/rust: rust.rs
	cargo test && cargo build --release

clean-rust:
	$(RM) -r rust rust-target
