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
        $systemPrompt = @"
You are a Windows power shell assistant. \
Given a user request, generate shell commands that fulfills the requirement. \
Before suggesting commands, use the provided tool to check if the commands exist on the user's system. \
Only respond with valid commands. \
Do not wrap, format, or explain the command—output only the command itself.
"@
        $aiResponse = & mobius exec --prompt "$afterAi" --system-prompt $systemPrompt
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
