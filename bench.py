from pyheck import snake, snake_many
from inflection import underscore


def test_snake(benchmark):
    benchmark(snake, "DeviceType")


def test_inflection_underscore(benchmark):
    benchmark(underscore, "DeviceType")


def test_snake_long_sentence(benchmark):
    benchmark(snake, "DeviceType" * 100_000)


def test_inflection_underscore_long_sentence(benchmark):
    benchmark(underscore, "DeviceType" * 100_000)


def test_snake_many(benchmark):
    benchmark(snake_many, ["DeviceType"] * 100_000)


def test_inflection_underscore_many(benchmark):
    benchmark(lambda lst: [underscore(x) for x in lst], ["DeviceType"] * 100_000)
