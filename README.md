# pyheck
PyHeck is a case conversion library (for converting strings to snake_case, camelCase etc). It is a thin wrapper around the Rust library [heck](https://github.com/withoutboats/heck).

[Read the documentation.](https://kevinheavey.github.io/pyheck/)
## Installation

```
pip install pyheck
```

## Example

```python
>>> from pyheck import snake
>>> snake("We carry a new world here, in our hearts.")
'we_carry_a_new_world_here_in_our_hearts'

```

## Development

### Setup

1. Install [poetry](https://python-poetry.org/)
2. Install dev dependencies:
```
poetry install
```
3. Activate the poetry shell:

```sh
poetry shell
```
### Testing
1. Run `maturin develop` to compile the Rust code.
2. Run `make fmt`, `make lint`, and `make test`.
