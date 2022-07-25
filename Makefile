.PHONY: all build systemd

all: install systemd

install: 
	cargo install --path .

systemd:
	cp etc/clipd.service $(HOME)/.config/systemd/user/
	systemctl --user daemon-reload
	systemctl --user enable clipd.service
	systemctl --user restart clipd.service

deb:
	cargo deb
	cp target/debian/*.deb ../ppa/
	cd ../ppa && \
		dpkg-scanpackages --multiversion . > Packages && \
		gzip -k -f Packages && \
		apt-ftparchive release . > Release && \
		gpg --default-key "wallbridge.calum@gmail.com" -abs -o - Release > Release.gpg && \
		gpg --default-key "wallbridge.calum@gmail.com" --clearsign -o - Release > InRelease
