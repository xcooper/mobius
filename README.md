# MOBIUS

A tool brings command line and AI models together.

## Get Started

Download the executable based on your OS.  Put it on a directory within the PATH.

Next, `mobius init` will help to set the configurations and save to the default location.

If you want to enable *AutoComplete*, you also need to execute **Mobius** in your shell init config.  Here are the instructions for different platforms.

### Windows

Edit the PowerShell config `$PROFILE` (e.g., `C:\Users\username\Document\PowerShell\Microsoft.PowerShell_profile.ps1`), add following line in the file.

```powershell
Invoke-Expression -Command (mobius auto-complete --shell power-shell | Out-String)
```

### Linux

Edit the shell init file based on your shell.

```bash
# add this to .bashrc
source <(mobius auto-complete --shell bash)
```

```bash
# add this to .zshrc
source <(mobius auto-complete --shell zsh)
```

## Commands

### Init

Set initial configurations.

```
Usage: mobius.exe init [OPTIONS] --provider <PROVIDER> --model <MODEL>

Options:
  -p, --provider <PROVIDER>  The AI provider to use [possible values: open-ai]
  -m, --model <MODEL>        The AI model to use, e.g., gpt-4o
  -a, --api-key <API_KEY>    The API key for accessing the AI provider
      --llm-url <LLM_URL>    The URL for accessing the AI provider, only need by in-house LLMs.
```

### Chat

Talk to the LLM with prompts and print out the response to the stdout.

```
Usage: mobius.exe chat [OPTIONS] --prompt <PROMPT>

Options:
  -p, --prompt <PROMPT>                The prompt, use '-' for reading from stdin
  -s, --system-prompt <SYSTEM_PROMPT>  The system prompt
```

### Shell Auto Complete

Like most auto-complete functions, this command meant to be triggered by `TAB` key with a special prefix `ai:` on your command line.  The returned commands from AI will consider the OS you are running.

See the "Get Started" section for the usage.

```
Usage: mobius.exe auto-complete [OPTIONS]

Options:
  -s, --shell <SHELL>  [possible values: zsh, bash, power-shell]
```

For example, `ai:find foobar in all files`, AI will properly return `grep -R foobar` on Linux.

> Note: The returned commands are based on public knowledges, you need to confirm them before you pressing ENTER.

## Config File

### Location

The config file will be one of the following locations.

* Linux & Mac
  * `$XDG_CONFIG_HOME/mobius/config.toml` (default `~/.config/mobius/config.toml`)
* Windows
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
- [ ] Support more LLM
  - [X] OpenAI
  - [ ] Gemini
