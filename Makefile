.PHONY: all build systemd

all: install systemd

install: 
	cargo install --path .

systemd:
	cp etc/clipd.service $(HOME)/.config/systemd/user/
	systemctl --user daemon-reload
	systemctl --user enable clipd.service