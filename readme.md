## ParseCfg  
A config file parser.

Please note that this is just a personal project and is developed as such.

ParseCfg (pronounced Parse-Config) is a parser library for `.ini`/`.cfg` files with support for
arrays, tuples and tables. Although ParseCfg may support some Toml features and may parse some
`.toml` files correctly, it does not try to support Toml's specification and should not be used for
Toml parsing.

### Structure
The structure of a config file is the same as ini, cfg and toml, but global keys are not allowed;
all keys must belong to a section. Names or IDs in ParseCfg are not case-sensitive, `KeyName` is the
same as `keyname`. All whitespace is ignored outside of strings so section headers and keys can be
split over several lines.

#### Documents
A `Document` represents a config file. A document contains a list of `Section`s that have unique,
case-insensitive names. Documents cannot have global keys, all keys must belong to a section. When
reading/writing from/to file, you would usually use a document.

#### Sections
A `Section` contains a list of `Key`s that have unique, case-insensitive names. Sections start with
their name, enclosed in square brackets, followed by the keys that belong to it.
```
[section]
# Section keys...
```

#### Keys
A `Key` is a name and value pair. Keys start with their name, followed by an equals sign `=`, then a
value.
```
name="value"
```

A keys' name must start with either a letter (a-z) or un underscore and can only contain letters,
digits (0-9) and underscores thereafter.

#### Values
A value enclosed in double quotes `"` will be interpreted as a string. If a string is followed by
another string, they will be concatenated together, allowing for multiline strings.

A value containing only digits (0-9) and up to one decimal point will be interpreted as a number. A
number that does not contains a decimal point will be interpreted as signed integer. A number that
contains a decimal point will be interpreted as a floating point number.

A numbers type can be declared explicitly using a letter postfix:
- `i` Forces a signed integer value.
- `u` Forces an unsigned integer value.
- `f` Forces a floating point value.

So `30.5u` would force the value to be an unsigned integer instead of a floating point number.

Arrays are lists of values which are all of the same type; they are declared by enclosing comma
separated values in brackets `[`, `]`. Arrays can only contain strings and number types, ParseCfg
does not support arrays containing arrays, tuples or tables.

Tuples are lists of values which can be of different types; they are declared by enclosing comma
separated values in parentheses `(`, `)`. Unlike an array, tuples can contain any type, including
arrays tables, and even tuples.

Tables are an anonymous lists of keys. Keys within a table do not need to have unique names like in
a section. A table is declared by by enclosing comma separated key declarations in braces `{`, `}`.

Possible Key Value Types
- `var="value"    # String.`
- `var="val" "ue" # Concatenated string.`
- `var=1          # Implicit signed integer.`
- `var=1.0        # Implicit floating point number.`
- `var=1.0i       # Explicit signed integer.`
- `var=1u         # Explicit unsigned integer.`
- `var=1f         # Explicit floating point number.`
- `var=[1,2,3]    # An array of integers.`
- `var=(1u,3f)    # A tuple containing an unsigned integer and a floating point number.`
- `var={n=3,m=2}  # A table containing the keys "n" and "m".`

### To Do
- Parse integral arithmetic.

## Changelog

### Version 0.2.0
- Added support for integral and floating point values so `Key`s can now contain numerical values
  with the `KeyValue` variants: `Integer`, `Unsigned` and `Float`.
- Arrays now support the new types with `KeyValue` variants `IntegerArray`, `UnsignedArray` and
  `FloatArray` allow arrays containing numerical values. `KeyValue::Array` has been renamed to 
  `KeyValue::StringArray` to reflect this.
- Added tuple values with `KeyValue` variant `Tuple` so `Key`s can now have lists of values with
  different types.
- Strings separated by whitespace are now automatically concatenated at the token level, for example
  `"Hel" "lo"` will produce a single string token `"Hello"`.

### Version 0.1.0
- Initial Release
