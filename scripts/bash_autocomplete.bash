command -v mobius >/dev/null 2>&1 || return 1

_mobius_completer() {
  # Get the command line up to the cursor position
  local line_before_cursor="${READLINE_LINE:0:$READLINE_POINT}"

  # Check if the text contains "ai:"
  if [[ "$line_before_cursor" == *ai:* ]]; then
    # Extract parts before and after "ai:"
    local before_ai="${line_before_cursor%%ai:*}"
    local after_ai="${line_before_cursor#*ai:}"

    # Run the mobius command with the text after "ai:"
    local ai_resp=$(mobius chat --prompt "$after_ai" --system-prompt "Be a Linux shell command assistant, only response with command, no wrappers, no format, be concise.")
    unwrapped_ai_resp=$ai_resp
    unwrapped_ai_resp=$(echo ${ai_resp} | sed -E 's/^\`*(bash\n)//; s/\n\`*$//')
    unwrapped_ai_resp=$(echo ${ai_resp} | sed -E 's/^\`*//; s/\`*$//')

    # Reconstruct the command line
    READLINE_LINE="${before_ai}${unwrapped_ai_resp}${READLINE_LINE:$READLINE_POINT}"
    # Place cursor after the inserted text
    READLINE_POINT=$((${#before_ai} + ${#unwrapped_ai_resp}))
  fi
}

# Bind Tab key to our custom function
bind -x '"\C-_": _mobius_completer'
