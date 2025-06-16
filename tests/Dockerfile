# This image is only for test purpose.
FROM rust:1.86.0-alpine
RUN apk add powershell; \
    apk add bash; \
    apk add zsh; \
	echo "source /scripts/zsh_autocomplete.zsh" >> ~/.zshrc; \
	echo "source /scripts/bash_autocomplete.bash" >> ~/.bashrc; \
	echo "Invoke-Expression -Command (mobius auto-complete --shell power-shell | Out-String)" >> ~/.config/powershell/Microsoft.PowerShell_profile.ps1
