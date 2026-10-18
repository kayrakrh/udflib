PREFIX ?= /usr
DESTDIR ?=

all:
	cargo build --release

install:
	install -Dm755 target/release/udfdump $(DESTDIR)$(PREFIX)/bin/udfdump

uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/udfdump

.PHONY: all install uninstall
