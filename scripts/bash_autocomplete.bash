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
    local ai_resp=$(mobius exec \
      --prompt "$after_ai" \
      --system-prompt "You are a Linux shell command assistant. \
Given a user request, generate shell commands that fulfills the requirement. \
Before suggesting commands, use the provided tool to check if the commands exist on the user's system. \
Only respond with valid commands. \
Do not wrap, format, or explain the command—output only the command itself.")
    unwrapped_ai_resp=$(echo ${ai_resp} | sed -E '/^`+[a-z]*$/d; /^`+$/d')

    # Reconstruct the command line
    READLINE_LINE="${before_ai}${unwrapped_ai_resp}${READLINE_LINE:$READLINE_POINT}"
    # Place cursor after the inserted text
    READLINE_POINT=$((${#before_ai} + ${#unwrapped_ai_resp}))
  fi
}

# Bind Ctrl+/ key to our custom function
bind -x '"\C-_": _mobius_completer'
