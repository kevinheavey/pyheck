from pyheck import snake, snake_many
from inflection import underscore


def test_snake(benchmark):
    benchmark(snake, "We are gonna make it")


def test_inflection_underscore(benchmark):
    benchmark(underscore, "We are gonna make it")


def test_snake_long_sentence(benchmark):
    benchmark(snake, "We are gonna make it" * 10_000)


def test_inflection_underscore_long_sentence(benchmark):
    benchmark(underscore, "We are gonna make it" * 10_000)


def test_snake_many(benchmark):
    benchmark(snake_many, ["We are gonna make it"] * 10_000)


def test_inflection_underscore_many(benchmark):
    benchmark(
        lambda lst: [underscore(x) for x in lst], ["We are gonna make it"] * 10_000
    )
