DESTDIR="/usr/local"

.PHONY: clean install

linuxsync:
	cargo build --release

install: linuxsync	
	install -D target/release/linuxsync $(DESTDIR)/bin/linuxsync
	cat man/linuxsync.1 | gzip > linuxsync.1.gz
	install -D --mode=644 linuxsync.1.gz  $(DESTDIR)/man/man1/linuxsync.1.gz

clean:
	cargo clean
