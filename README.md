# MOBIUS

A tool that integrates command-line interfaces with AI models.

## Get Started

Download the executable for your OS and place it in a directory included in your PATH.

Run `mobius init` to configure the tool and save the settings to the default location.

If you need a free API key, please try [Gemini](https://aistudio.google.com/apikey).
For example,
```bash
mobius init --provider gemini --api-key AIza... --model gemini-2.0-flash
```

### Shell Auto Complete

To enable *AutoComplete*, add **Mobius** to your shell initialization file. Instructions for different platforms are
below:

#### Windows

Edit your PowerShell profile `$PROFILE` (e.g.,
`C:\Users\username\Documents\PowerShell\Microsoft.PowerShell_profile.ps1`) and add:

```powershell
Invoke-Expression -Command (mobius auto-complete --shell power-shell | Out-String)
```

#### Linux

Edit the shell initialization file for your shell:

```bash
# For Bash, add this to .bashrc
source <(mobius auto-complete --shell bash)
```

```bash
# For Zsh, add this to .zshrc
source <(mobius auto-complete --shell zsh)
```

## Commands

### Init

Set up initial configurations.

```
Usage: mobius.exe init [OPTIONS] --provider <PROVIDER> --model <MODEL>

Options:
  -p, --provider <PROVIDER>  The AI provider to use [possible values: open-ai]
  -m, --model <MODEL>        The AI model to use, e.g., gpt-4o
  -a, --api-key <API_KEY>    The API key for accessing the AI provider
      --llm-url <LLM_URL>    The URL for accessing the AI provider, only need by in-house LLMs.
```

### Chat

Interact with the LLM using prompts and display the response in the terminal.

```
Usage: mobius.exe chat [OPTIONS] --prompt <PROMPT>

Options:
  -p, --prompt <PROMPT>                The prompt, use '-' for reading from stdin
  -s, --system-prompt <SYSTEM_PROMPT>  The system prompt
```

### Shell Auto Complete

Enable AI-powered auto-completion triggered by the `TAB` key with the prefix `ai:`. The AI-generated commands will
consider your operating system.

Refer to the "Get Started" section for setup instructions.

```
Usage: mobius.exe auto-complete [OPTIONS]

Options:
  -s, --shell <SHELL>  [possible values: zsh, bash, power-shell]
```

Example: Typing `ai:find foobar in all files` will return `grep -R foobar` on Linux.

> Note: The AI-generated commands are based on public knowledge. Verify them before pressing ENTER.

## Config File

### Location

The configuration file is located in one of the following paths:

* Linux & Mac:
    * `$XDG_CONFIG_HOME/mobius/config.toml` (default: `~/.config/mobius/config.toml`)
* Windows:
  * `%APPDATA%\mobius\config.toml`

## Features

- [X] Let AI processing the input and produce meaningful output
- [X] Ask AI generating a command based your prompt
- [ ] Call other commands based on AI response
- [ ] Manage prompts
- [ ] Trigger AI generating a command using auto-complete(Tab key)
  - [X] Windows PowerShell
  - [X] Linux ZSH
  - [X] Linux BASH
  - [ ] MAC ZSH
- [X] Support more LLM
  - [X] OpenAI
  - [X] Gemini
