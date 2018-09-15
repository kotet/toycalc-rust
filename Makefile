build/toycalc: toycalc.rs
	mkdir -p build
	rustc toycalc.rs -o build/toycalc

.PHONY: clean test
clean:
	rm -r build

test: build/toycalc
	sh test.sh
