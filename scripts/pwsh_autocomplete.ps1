# Check if mobius command exists
if (-not (Get-Command mobius -ErrorAction SilentlyContinue)) {
    return
}

# Register the tab key handler
function global:OnTabPressed {
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
        $aiResponse = & mobius chat --prompt "$afterAi" --system-prompt "Be a Windows power shell assistant, only response with command, no wrappers, no format, be concise."
        
        # Replace the current line up to cursor with transformed content
        $newLine = $beforeAi + $aiResponse
        
        # Update the current line
        [Microsoft.PowerShell.PSConsoleReadLine]::Replace(0, $cursor, $newLine)
    }
    else {
        # Default tab completion behavior
        [Microsoft.PowerShell.PSConsoleReadLine]::TabCompleteNext($key, $arg)
    }
}

# Set the PSReadLine key handler
Set-PSReadLineKeyHandler -Key Tab -ScriptBlock $function:OnTabPressed