# MOBIUS

A tool brings command line and AI models together.

## Features

- [V] Let AI processing the input and produce meaningful output
- [V] Ask AI generating a command based your prompt
- [ ] Call other commands based on AI response
- [ ] Manage prompts
- [ ] Trigger AI generating a command using auto-complete(Tab key)
  - [V] Windows PowerShell
  - [V] Linux ZSH
  - [V] Linux BASH
  - [ ] MAC ZSH
- [ ] Support more LLM
  - [V] OpenAI
  - [ ] Gemini

## Get Started

Firstly, you need to configure **Mobius**.

* The AI model you going to use.
  * API key for accessing AI.
  * The model going to use.
* Some prompts saved for future use.

The configuration file honor the standard location based on your OS.
`mobius init` will generate the default configuration and save to the default location.

## Commands

### Init(TODO)

```shell
mobius init
```

This is a simple command only initialize some configurations.  Including,
* The configurations for using AI

### Pipe(TODO)

```shell
cat Main.java | mobius pipe --llm openai --prompt "rewrite the Java code in Rust" | tee main.rs
```

This command sends the prompt with the stdin to AI and returns the result.

```shell
cat data.csv | mobius pipe --prompt-file ./csv-to-json-prompt.txt | jq '.name'
```

Also supports reading prompts from files.

### Complete(TODO)

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

### Config Structure

The example of configuration is

```toml
[llm]
provider="openai"
model="gpt-4o-2024-08-06"
api_key="<openai api key>"
```
