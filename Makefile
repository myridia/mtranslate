.PHONY: api

dev:
	cargo watch -q -c -w src/ -x run

release:
	git pull;\
	cargo build --release 
default: dev

