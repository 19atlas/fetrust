alias b := build
alias i := install
alias u := uninstall
alias n := native_install

# build
build:
	cargo build --release

# build and install
install: 
	cargo install --path .

# optimize for native cpu and install
native_install:
  RUSTFLAGS="-C target-cpu=native" cargo install --path .

# uninstall
uninstall:
	cargo uninstall fetrust
