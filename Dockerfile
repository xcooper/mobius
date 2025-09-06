FROM rust:1.89.0 AS builder
ARG PWSH_DNLD_URL=https://github.com/PowerShell/PowerShell/releases/download/v7.4.11/powershell-7.4.11-linux-arm64.tar.gz
# workaround, this is a bug
ENV HOME=/root
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
	--mount=type=cache,target=/var/lib/apt,sharing=locked \
	set -xeo; \
	mkdir /root/pwsh; \
	curl -L ${PWSH_DNLD_URL} | tar zx -C /root/pwsh/; \
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
RUN --mount=type=cache,target=/root/.cargo \
	cd /src; \
	cargo install --path=.; \
	mkdir -p ~/.config/mobius/; \
	cp /src/target/config.toml ~/.config/mobius/config.toml
