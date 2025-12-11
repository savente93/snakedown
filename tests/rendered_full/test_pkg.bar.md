# test_pkg.bar

bar.py

Example module with a function and a class.

## test_pkg.bar.greet

greet(name: str) -> str

Return a greeting message.

Args:
    name (str): Name to greet.

Returns:
    str: Greeting message.

Example:
    >>> greet("Anna")
    'Hello, Anna!'

## test_pkg.bar.greet_undocumented

greet_undocumented(name)

## test_pkg.bar._format_name

_format_name(name: str) -> str

Format the name string to title case (private helper).

Args:
    name (str): Name string.

Returns:
    str: Formatted name.

## test_pkg.bar.Greeter

Greeter class that holds a name and greets.

Attributes:
    name (str): The name to greet.

### test_pkg.bar.Greeter.__init__

__init__(self, name: str)

Initialize with a name.

Args:
    name (str): Name to greet.

### test_pkg.bar.Greeter.greet

greet(self) -> str

Generate a greeting message.

Returns:
    str: Greeting message.
