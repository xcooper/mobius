FROM rust:1.89.0 AS builder
ARG PWSH_DNLD_URL=https://github.com/PowerShell/PowerShell/releases/download/v7.5.3/powershell-7.5.3-linux-arm64.tar.gz
ENV RUST_BACKTRACE=1
ENV DEBUG=1
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
	echo "source <(mobius auto-complete --shell zsh --hot-key ctrl-slash)" >> ~/.zshrc; \
	echo "source <(mobius auto-complete --shell bash --hot-key ctrl-slash)" >> ~/.bashrc; \
	mkdir -p ~/.config/powershell/; \
	echo "Invoke-Expression -Command (mobius auto-complete --shell power-shell --hot-key alt-slash | Out-String)" >> ~/.config/powershell/Microsoft.PowerShell_profile.ps1;
COPY . /src
RUN --mount=type=cache,target=/root/.cargo \
	cd /src; \
	cargo install --path=.; \
	mkdir -p ~/.config/mobius/; \
	cp /src/target/config.toml ~/.config/mobius/config.toml
