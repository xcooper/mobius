[ $(command -v mobius) ] || return 1

# Detect operating system and set OS environment variable
if [[ "$OSTYPE" == "darwin"* ]]; then
  export OS=macos
  # Export macOS version information
  if command -v sw_vers >/dev/null 2>&1; then
    export OS_RELEASE=$(sw_vers -productName)
  fi
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
  export OS=linux
  # Export Linux release name if os-release file exists
  if [[ -f /etc/os-release ]]; then
    export OS_RELEASE=$(. /etc/os-release && echo $NAME)
  fi
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" || "$OSTYPE" == "win32" ]]; then
  export OS=windows
  # Export Windows OS name if systeminfo command exists
  if command -v systeminfo >/dev/null 2>&1; then
    export OS_RELEASE=$(systeminfo | grep -i "OS Name:" | sed 's/^.*://; s/^ *//')
  fi
else
  echo "Your OS is not supported: $OSTYPE" >&2
  return 255
fi

# Set USER to 'root user' if UID is 0, otherwise 'user'
if [[ "$UID" == "0" ]]; then
  export USER="root user"
else
  export USER="user"
fi

_mobius_completer() {
  local line_before_cursor="${BUFFER:0:$CURSOR}"
  if [[ "$line_before_cursor" == *ai:* ]]; then
    local before_ai=${line_before_cursor##ai:*}
    local after_ai=${line_before_cursor##*ai:}
  local ai_resp=$(mobius exec \
    --prompt "$after_ai" \
    --system-prompt "You are a Zsh command assistant. \
Given a user request, generate one or more shell commands that fulfill the requirement. \
Separate multiple commands with semicolons (;) on a single line. \
You MUST use the 'check_cmd_exist' tool to verify that all commands are available and valid for $OS($OS_RELEASE) before suggesting them. \
Only respond with valid commands for a $USER. \
Do not wrap in code blocks, format, or explain - output only the command(s) themselves. \
Let's do this step by step. \
")
    unwrapped_ai_resp=$(echo ${ai_resp} | sed -E '/^`+[a-z]*$/d; /^`+$/d')
    BUFFER="${before_ai}${unwrapped_ai_resp}"
  fi
}

zle -N mobius_completer _mobius_completer
# Bind key to our custom function based on MOBIUS_KEY_BINDING env var
# Default is Ctrl+/ (^_), alternative is Alt+/ (\e/)
if [[ "${MOBIUS_KEY_BINDING}" == "ALT_SLASH" ]]; then
  bindkey '\e/' mobius_completer
else
  bindkey '^_' mobius_completer
fi
