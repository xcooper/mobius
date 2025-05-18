[ $(command -v mobius) ] || return 1

_mobius_completer() {
  local line_before_cursor="${BUFFER:0:$CURSOR}"
  if [[ "$line_before_cursor" == *ai:* ]]; then
    local before_ai=${line_before_cursor##ai:*}
    local after_ai=${line_before_cursor##*ai:}
    local ai_resp=$(mobius chat --prompt "$after_ai" --system-prompt "Be a Linux shell command assistant, only response with command, no wrappers, no format, be concise.")
    unwrapped_ai_resp=$ai_resp
    unwrapped_ai_resp=$(echo ${ai_resp} | sed -E 's/^\`*(bash\n)//; s/\n\`*$//')
    unwrapped_ai_resp=$(echo ${ai_resp} | sed 's/^\`*//; s/\`*$//')
    BUFFER="${before_ai}${unwrapped_ai_resp}"
  fi
}

zle -N mobius_completer _mobius_completer
bindkey '^_' mobius_completer
