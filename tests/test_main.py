from collections import deque
from pyheck import kebab, kebab_many

SINGLE_INPUT = "We are going to inherit the earth."
SINGLE_OUTPUT = "we-are-going-to-inherit-the-earth"
MULTIPLE_INPUTS = ["We are going", "We are leaving"]
MULTIPLE_OUTPUTS = ["we-are-going", "we-are-leaving"]


def test_kebab():
    assert kebab(SINGLE_INPUT) == SINGLE_OUTPUT


def test_kebab_many():
    assert kebab_many(MULTIPLE_INPUTS) == MULTIPLE_OUTPUTS


def test_tuple():
    assert kebab_many(tuple(MULTIPLE_INPUTS)) == MULTIPLE_OUTPUTS


def test_deque():
    assert kebab_many(deque(MULTIPLE_INPUTS)) == MULTIPLE_OUTPUTS
