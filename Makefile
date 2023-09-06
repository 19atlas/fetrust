.PHONY: all build install uninstall
all: $(build) $(install)

build:
	cargo build --release

install:
	@if [ "x$(shell id -u)" = "x0" ]]; then\
		mv ./target/release/fetrust /usr/local/bin;\
	else\
		echo pls run under sudo or doas;\
	fi
uninstall:
	@if [ "x$(shell id -u)" = "x0" ]; then\
			rm -v /usr/local/bin/fetrust;\
		else\
			echo bro please run me under sudo or doas;\
		fi
