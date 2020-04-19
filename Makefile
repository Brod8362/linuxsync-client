linuxsync:
	cargo build --release

install:
	cp target/release/linuxsync /usr/bin/linuxsync
	cat man/linuxsync.1 | gzip > /usr/share/man/man1/linuxsync.1.gz

clean:
	cargo clean