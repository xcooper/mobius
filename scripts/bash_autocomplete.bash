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
        
        # Reconstruct the command line
        READLINE_LINE="${before_ai}${ai_resp}${READLINE_LINE:$READLINE_POINT}"
        # Place cursor after the inserted text
        READLINE_POINT=$((${#before_ai} + ${#ai_resp}))
    else
        # Default to normal tab completion
        bind '"\C-i": complete'
    fi
}

# Bind Tab key to our custom function
bind -x '"\C-i": _mobius_completer'