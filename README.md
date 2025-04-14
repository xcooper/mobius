# MOBIUS

A tool brings command line and AI models together.

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

## Get Started

Firstly, you need to configure **Mobius**.

* The AI model you going to use.
  * API key for accessing AI.
  * The model going to use.
* Some prompts saved for future use.

The configuration file honor the standard location based on your OS.
`mobius init` will generate the default configuration and save to the default location.

If you want to enable *AutoComplete*, you need to execute **Mobius** in your shell init config.

### Windows
Edit the PowerShell config `$PROFILE`, add following line in the file.

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

```shell
mobius init --provider open-ai --model gpt-3.5-turbo --api-key ...
```

This is a simple command only initialize some configurations.

### Chat

```shell
cat Main.java | mobius chat --prompt "rewrite the Java code in Rust" | tee main.rs
```

Also supports reading prompts from files.

### Shell Auto Complete

Like most auto-complete functions, this command meant to be triggered on <tab> key with a special 
prefix `ai:` on your command line.

```shell
> ai: use "grep" to search "foobar" in all files<tab>
```

AI will properly return `grep -R foobar`.

## Config File

### Location

The config file will be one of the following locations.

* Linux & Mac
  * `$XDG_CONFIG_HOME/mobius/config.toml` (default `~/.config/mobius/config.toml`)
* Windows
  * `%APPDATA%\mobius\config.toml`
