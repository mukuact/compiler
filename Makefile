
cc1: cc1.rs
	rustc cc1.rs

test: cc1
	./test.sh

clean:
	rm -f cc1 tmp* *~ 

.PHONY: test clean

