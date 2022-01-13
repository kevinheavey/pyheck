from multiprocessing import Pool, cpu_count
from pyheck import snake
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
    benchmark(snake, "DeviceType")


def test_inflection_underscore(benchmark):
    benchmark(underscore, "DeviceType")


def test_snake_long_sentence(benchmark):
    benchmark(snake, "DeviceType" * 100_000)


def test_inflection_underscore_long_sentence(benchmark):
    benchmark(underscore, "DeviceType" * 100_000)


def test_inflection_underscore_many(benchmark):
    benchmark(lambda lst: [underscore(x) for x in lst], ["DeviceType"] * 100_000)


def test_snake_multiprocessing(benchmark):
    benchmark(snake_multiprocessing, ["DeviceType"] * 100_000)


def test_inflection_underscore_multiprocessing(benchmark):
    benchmark(inflection_underscore_multiprocessing, ["DeviceType"] * 100_000)
