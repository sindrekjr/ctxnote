# ctxnote

The mind is a mess. Get your thoughts into contexts!

This is a simple command line tool for taking down notes as they come while working on the command line, without the hassle of having to switch out of your flow. And most importantly, easily put notes into appropriate contexts.

Nothing is implemented yet; all that follows is intention.

## CLI Structure

```
|- note
|   | <entry> [-c <context>]
|   |- init (alias: note ctx init)
|   |- get <pattern> [-c <context>]
|   |- conf [-c <context>]
|   |   |- get <key>
|   |   |- set <key> <val>
|   |- ctx
|   |   | <name>
|   |   |- init [-n <name>] [-b]
|   |   |- get <pattern>
|   |   |- rm <name>
|   |   |- mv <name> <newname>
```
