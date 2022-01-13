# pyheck
Python bindings for heck, the Rust case conversion library.

## Installation

```
pip install pyheck
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
