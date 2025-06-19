# This image is only for test purpose.
FROM rust:1.86.0 as builder
# workaround, this is a bug
ENV HOME=/root
RUN apk add powershell; \
    apk add bash; \
    apk add zsh; \
	echo "source <(mobius auto-complete --shell zsh)" >> ~/.zshrc; \
	echo "source <(mobius auto-complete --shell bash)" >> ~/.bashrc; \
	mkdir -p ~/.config/powershell/; \
	echo "Invoke-Expression -Command (mobius auto-complete --shell power-shell | Out-String)" >> ~/.config/powershell/Microsoft.PowerShell_profile.ps1;
COPY . /src
RUN cd /src; \
	cargo install --path=.; \
	mkdir -p ~/.config/mobius/; \
	cp /src/target/config.toml ~/.config/mobius/config.toml;
