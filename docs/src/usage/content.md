# Writing Content

Snakedown can extract a variety of information from you python package, including type signatures and docstrings. The output is structured similarly to that of Sphinx, meaning that all "objects" (e.g. functions, classes and modules) get their own page in a flat directory structure.

## Linking

Snakedown introduces a lightweight syntax for linking to other objects that is inspired by the one that [Obsidian](https://obsidian.md) uses, namelly `[[ fully.qualified.name ]]` this will then get turned into the correct link in whatever format your supported static site generator expects. For the moment only fully qualified references are supported, meaning that you can only reference them via their full import path. You can use this syntax anywhere in your docstrings.

You can also provide some optional display text by using the `|` character like so: `[[ foo.bar.baz | The baz module]]` which will be used as the text for the generated link. If you do not provide any, the reference target name will be used (i.e. `[[ foo.bar ]]` will be changed to `[foo.bar](foo.bar.md)` but `[[ foo.bar | the bar module]]` would be changed to `[the bar module](foo.bar.md)`)
