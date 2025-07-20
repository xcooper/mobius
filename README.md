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

Enable AI-powered auto-completion triggered by the `CTRL+/` key with the prefix `ai:`. The AI-generated commands will
consider your operating system.

Example: Typing `ai:find foobar in all files` will return `grep -R foobar` on Linux.

> Note: The AI-generated commands are based on public knowledge. Verify them before pressing ENTER.
> Powershell on Linux is not supported.

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

### Chat

The `chat` command allows you to have a conversation with the AI. You can provide your message directly as an argument or pipe it from another command. This is useful for asking questions, generating text, or getting help with various tasks.

For example, you can ask a simple question like this:
```bash
mobius chat --prompt "What is the capital of France?"
```

You can also use it with other commands. For example, to ask the AI to explain a command's output:
```bash
ls -l | mobius chat --prompt "Explain what these file permissions mean"
```

## Config File

### Location

The configuration file is located in one of the following paths:

* Linux & Mac:
    * `$XDG_CONFIG_HOME/mobius/config.toml` (default: `~/.config/mobius/config.toml`)
* Windows:
  * `%APPDATA%\mobius\config.toml`

## TODOs

- [X] Let AI processing the input and produce meaningful output
- [X] Ask AI generating a command based your prompt
- [ ] Have a tool checking the existence of commands on user's local.
- [ ] Call other commands based on AI response
- [ ] Manage prompts
- [X] Trigger AI generating a command using auto-complete(CTRL+/ key)
  - [X] Windows PowerShell
  - [X] Linux ZSH
  - [X] Linux BASH
  - [X] MAC ZSH
- [X] Support more LLM
  - [X] OpenAI
  - [X] Gemini
