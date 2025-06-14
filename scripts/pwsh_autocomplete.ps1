# Check if mobius command exists
if (-not (Get-Command mobius -ErrorAction SilentlyContinue)) {
    return
}

# Register the tab key handler
function global:TriggerMobius {
    param($key, $arg)

    # Get the line content and cursor position
    $line = $null
    $cursor = $null
    [Microsoft.PowerShell.PSConsoleReadLine]::GetBufferState([ref]$line, [ref]$cursor)
    
    # Get the line before cursor
    $lineCursor = $line.Substring(0, $cursor)
    
    # Check if the line contains "ai:" pattern
    if ($lineCursor -match "ai:") {
        $beforeAi = $lineCursor.Substring(0, $lineCursor.LastIndexOf("ai:"))
        $afterAi = $lineCursor.Substring($lineCursor.LastIndexOf("ai:") + 3)
        
        # Process with mobius pipe
        $aiResponse = & mobius chat \
                    --prompt "$afterAi" \
                    --system-prompt "Be a Windows power shell assistant, \
                        only response with command, \
                        no wrap, no format, be concise."
        $trimmedAiResponse = $aiResponse -replace "(?s)^```[a-zA-Z0-9]*\\n?", ''
        $trimmedAiResponse = $trimmedAiResponse -replace "```$", ''
        
        # Replace the current line up to cursor with transformed content
        $newLine = $beforeAi + $trimmedAiResponse
        
        # Update the current line
        [Microsoft.PowerShell.PSConsoleReadLine]::Replace(0, $cursor, $newLine)
    }
}

# Set the PSReadLine key handler
Set-PSReadLineKeyHandler -Chord 'Ctrl+/' -ScriptBlock $function:TriggerMobius
