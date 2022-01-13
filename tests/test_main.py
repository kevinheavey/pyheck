from pyheck import kebab, snake

SINGLE_INPUT = "We are going to inherit the earth."
SINGLE_OUTPUT = "we-are-going-to-inherit-the-earth"


def test_kebab():
    assert kebab(SINGLE_INPUT) == SINGLE_OUTPUT


def test_unicode():
    assert snake("XΣXΣ baﬄe") == "xσxς_baﬄe"
