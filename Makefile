.PHONY: build systemd

build: 
	cargo install --path .

systemd: build
	cp systemd/clipd.service $(HOME)/.config/systemd/user/
	systemctl --user daemon-reload
	systemctl --user enable clipd.service