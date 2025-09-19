# Check if mobius command exists
if (-not (Get-Command mobius -ErrorAction SilentlyContinue)) {
    return
}

# Detect operating system and set OS environment variable
if ($PSVersionTable.PSVersion.Major -ge 6) {
    # PowerShell Core (6+) - cross-platform
    if ($IsMacOS) {
        $env:OS = "macos"
    } elseif ($IsLinux) {
        $env:OS = "linux"
    } elseif ($IsWindows) {
        $env:OS = "windows"
    } else {
        Write-Error "Unsupported operating system detected"
        return
    }
} else {
    # Windows PowerShell (5.1 and below) - Windows only
    $env:OS = "windows"
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
You are a PowerShell assistant. Given a user request, generate one or
more shell commands that fulfill the requirement. Separate multiple
commands with semicolons (;) on a single line. You MUST use the
'check_cmd_exist' tool to verify that all commands are available and
valid for $env:OS before suggesting
them. Only respond with valid commands OR PowerShell built-in commands. Do not wrap in code blocks,
format, or explain - output only the command(s) themselves.
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
Set-PSReadLineKeyHandler -Chord 'Alt+/' -ScriptBlock $function:TriggerMobius
