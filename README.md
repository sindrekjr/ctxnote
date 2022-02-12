# ctxnote

The mind is a mess. Get your thoughts into contexts!

This is a simple command line tool for taking down notes as they come while working on the command line, without the hassle of having to switch out of your flow. And most importantly, easily put notes into appropriate contexts.

⚠️ WIP. Everything that follows is intention and may not be implemented yet.

## CLI Structure

```
|- note [-c <context>]
|   |- add <entry>
|   |- get <pattern>
|   |- init (alias: note ctx init)
|   |- conf
|   |   |- get <key>
|   |   |- set <key> <val>
|   |- ctx
|   |   | <name>
|   |   |- init [-n <name>] [-b]
|   |   |- get <pattern>
|   |   |- rm <name>
|   |   |- mv <name> <newname>
```

## Usage

### Options

#### `-c` `--context`
Sets context inline.

### `note add`
```
note add <entry>
```
Adds a note entry to the current context.

<details>
  <summary>Examples</summary>

  ```
  $ note add "this is an example entry"
  [default] added entry: this is an example entry
  ```
  ```
  $ note add "this is an example entry in another context" -c examples
  [examples] added entry: this is an example entry in another context
  ```
</details>

### `note get`
```
note get <pattern>
```
Gets all entries that correspond to the given pattern.
