build:
	cargo build
release:
	cargo publish
test:
	./tests/setup.sh && ./tests/accounts_test.sh && ./tests/transactions_test.sh
