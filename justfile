alias b := build
alias i := install
alias u := uninstall

# build
build:
	cargo build --release

# build and install
install: 
	cargo install --path .

# uninstall
uninstall:
	cargo uninstall fetrust
