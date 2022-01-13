from multiprocessing import Pool, cpu_count
from pyheck import snake, snake_many
from inflection import underscore


def inflection_underscore_multiprocessing(strings: list[str]) -> list[str]:
    try:
        workers = cpu_count()
    except NotImplementedError:
        workers = 1
    pool = Pool(processes=workers)
    return pool.map(underscore, strings)


def snake_multiprocessing(strings: list[str]) -> list[str]:
    try:
        workers = cpu_count()
    except NotImplementedError:
        workers = 1
    pool = Pool(processes=workers)
    return pool.map(snake, strings)


def test_snake(benchmark):
    val = "DeviceType"
    benchmark(snake, val)


def test_inflection_underscore(benchmark):
    val = "DeviceType"
    benchmark(underscore, val)


def test_snake_long_sentence(benchmark):
    val = "DeviceType" * 100_000
    benchmark(snake, val)


def test_inflection_underscore_long_sentence(benchmark):
    val = "DeviceType" * 100_000
    benchmark(underscore, val)


def test_inflection_underscore_many(benchmark):
    val = ["DeviceType"] * 100_000
    benchmark(lambda lst: [underscore(x) for x in lst], val)


def test_snake_multiprocessing(benchmark):
    val = ["DeviceType"] * 100_000
    benchmark(snake_multiprocessing, val)


def test_snake_many(benchmark):
    val = ["DeviceType"] * 100_000
    benchmark(snake_many, val)


def test_inflection_underscore_multiprocessing(benchmark):
    val = ["DeviceType"] * 100_000
    benchmark(inflection_underscore_multiprocessing, val)
