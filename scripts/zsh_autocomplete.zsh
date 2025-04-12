[ $(command -v mobius) ] || return 1

_mobius_completer() {
    local line_before_cursor="${BUFFER:0:$CURSOR}"
    if [[ "$line_before_cursor" == *ai:* ]]; then
        local before_ai=${line_before_cursor##ai:*}
        local after_ai=${line_before_cursor##*ai:}
        local ai_resp=$(mobius pipe -p "$after_ai")
        BUFFER="${before_ai}${ai_resp}"
    else
        zle expand-or-complete
    fi
}

zle -N mobius_completer _mobius_completer
bindkey '^I' mobius_completer