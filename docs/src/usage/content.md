# Writing Content

Snakedown can extract a variety of information from you python package, including type signatures and docstrings. The output is structured similarly to that of Sphinx, meaning that all "objects" (e.g. functions, classes and modules) get their own page in a flat directory structure.

## Linking

Snakedown introduces a lightweight syntax for linking to other objects that is inspired by the one that [Obsidian](https://obsidian.md) uses, namelly `[[ fully.qualified.name ]]` this will then get turned into the correct link in whatever format your supported static site generator expects. For the moment only fully qualified references are supported, meaning that you can only reference them via their full import path. You can use this syntax anywhere in your docstrings.

You can also provide some optional display text by using the `|` character like so: `[[ foo.bar.baz | The baz module]]` which will be used as the text for the generated link. If you do not provide any, the reference target name will be used (i.e. `[[ foo.bar ]]` will be changed to `[foo.bar](foo.bar.md)` but `[[ foo.bar | the bar module]]` would be changed to `[the bar module](foo.bar.md)`)

## Jupyter Notebooks

Snakedown now supports including the output of jupyter notebooks in your documentation. Currently only python notebooks are supported. This is more out of consistency because python is the only language we currently parse, so it doesn't make much sense to allow for notebooks in other languages, however, this could change in the future.

The output of a notebook being processed by snakedown will be a markdown file with the same file stem as the notebook. This will contain all the markdown cells included as is. Code cells will be included wrapped in a python code block. Output of supported formats will be included as well. If there is image data, this will be included as a colocated file which will be referenced in the output.

We plan to add support for expanding references like in the section above, though likely only markdown cells will be expanded.


### Why is my Jupyter notebook output not showing up?

The Jupyter format can actually output a surprisng amount of different kinds of output, many of those we didn't have an example case for, and as a general rule we don't implement things we can't verify the use of. However, if you have something that produces output we don't support and are willing to share (a simplified version) so we can make sure it works properly, please open a feature request. The media types that our dependency support are listed [here](https://docs.rs/jupyter-protocol/1.0.0/jupyter_protocol/media/enum.MediaType.html).
