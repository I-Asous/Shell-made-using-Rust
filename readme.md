# ShellInRust

A basic Unix shell implemented in Rust, built as a learning project.

## Build & Run

```sh
cargo build
cargo run
```

## Usage

Type commands at the `> ` prompt, one per line.

```
> ls
> echo hello
> ls | grep Cargo
> cd ..
> exit
```

## Features

- **Command execution** — runs any executable found on `PATH`, passing along arguments.
- **Piping** — chain commands with `|` (e.g. `ls | grep foo`); supports multi-stage pipelines, not just two commands.
- **Built-ins**:
  - `cd [dir]` — change directory (defaults to `/` if no argument is given)
  - `exit` — quit the shell

## Known Limitations

- **No I/O redirection** — `>`, `>>`, and `<` are not parsed; they're passed through as literal arguments to the command.
- **No background jobs** — `&` is not supported, and there are no `fg`/`bg` builtins.
- **No signal handling** — pressing Ctrl+C terminates the whole shell (and any running child), rather than just interrupting the foreground command.
- Only `cd` and `exit` are true built-ins; everything else (including `echo`, `pwd`, `ls`) is a subprocess call to the corresponding system binary.
