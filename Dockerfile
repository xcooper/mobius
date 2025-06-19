# This image is only for test purpose.
FROM rust:1.86.0 as builder
# workaround, this is a bug
ENV HOME=/root
RUN \
	mkdir /root/pwsh; \
	curl -L https://github.com/PowerShell/PowerShell/releases/download/v7.4.10/powershell-7.4.10-linux-arm64.tar.gz | tar zx -C /root/pwsh/; \
	chmod +x /root/pwsh/pwsh; \
	ln -s /root/pwsh/pwsh /usr/bin/pwsh; \
	apt-get update; \
    apt-get install -y bash; \
    apt-get install -y zsh; \
	echo "source <(mobius auto-complete --shell zsh)" >> ~/.zshrc; \
	echo "source <(mobius auto-complete --shell bash)" >> ~/.bashrc; \
	mkdir -p ~/.config/powershell/; \
	echo "Invoke-Expression -Command (mobius auto-complete --shell power-shell | Out-String)" >> ~/.config/powershell/Microsoft.PowerShell_profile.ps1;
COPY . /src
RUN cd /src; \
	cargo install --path=.; \
	mkdir -p ~/.config/mobius/; \
	cp /src/target/config.toml ~/.config/mobius/config.toml;
