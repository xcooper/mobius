[ $(command -v mobius) ] || return 1

_mobius_completer() {
  local line_before_cursor="${BUFFER:0:$CURSOR}"
  if [[ "$line_before_cursor" == *ai:* ]]; then
    local before_ai=${line_before_cursor##ai:*}
    local after_ai=${line_before_cursor##*ai:}
  local ai_resp=$(mobius exec \
    --prompt "$after_ai" \
    --system-prompt "You are a Linux shell command assistant. \
Given a user request, generate shell commands that fulfills the requirement. \
Before suggesting commands, use the provided tool to check if the commands exist on the user's system. \
Only respond with valid commands. \
Do not wrap, format, or explain the command—output only the command itself.")
    unwrapped_ai_resp=$(echo ${ai_resp} | sed -E '/^`+[a-z]*$/d; /^`+$/d')
    BUFFER="${before_ai}${unwrapped_ai_resp}"
  fi
}

zle -N mobius_completer _mobius_completer
bindkey '^_' mobius_completer
