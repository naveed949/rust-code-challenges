# Makefile for Rust web server project

# Variables
CARGO = cargo
SRC = src/**/*.rs

# Basic commands
all: build

build:
	$(CARGO) build

run:
	$(CARGO) fmt && $(CARGO)  run

test:
	$(CARGO) test

clean:
	$(CARGO) clean

# Command to keep server reloading upon save
watch:
	cargo watch -q -c -w src/ -x 'fmt'

# Command to auto reformat the code
fmt:
	$(CARGO) fmt

# Phony targets
.PHONY: all build run test clean watch fmt 