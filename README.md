# pyheck
Python bindings for heck, the Rust case conversion library.

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
