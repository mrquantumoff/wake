buildandclean:
	cargo clean
	cargo build
install:
	echo "Root required!"
	su -c 'install -Dm755 -t /bin/ target/debug/wake && chmod +x /bin/wake'
full:
	make
	make install