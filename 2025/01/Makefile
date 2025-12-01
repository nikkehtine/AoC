TARGET = aoc2025_01

all: build

build:
	rustc main.rs -o bin

run: build
	./bin input.txt

test: build
	./bin example.txt

clean:
	rm -f bin
	rm -f output.txt
