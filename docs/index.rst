.. PyHeck documentation master file, created by
   sphinx-quickstart on Thu Jan 13 12:02:58 2022.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

PyHeck
======

PyHeck is a case conversion library.
It exists to provide case conversion between common cases like CamelCase and snake_case.
It is intended to be unicode aware, internally, consistent, and reasonably well performing.

PyHeck is merely a wrapper around a Rust library `heck <https://docs.rs/heck/latest/heck/>`_.
Most of this documentation is copied lovingly from the ``heck`` docs.

.. note::
    Each ``pyheck`` function has a ``_many`` counterpart that operates on a sequence of strings.
    For example, the ``snake`` function converts a single string to snake case, while
    ``snake_many`` converts a sequence of strings into a list of snake case strings.
    The ``_many`` functions achieve high performance by taking advantage of Rust's parallel features,
    so you should use them in places where they make sense.

Installation
------------

::

    pip install pyheck

API
---

.. automodule:: pyheck
    :imported-members:
    :members:
    :undoc-members:

.. toctree::
   :maxdepth: 2
   :caption: Contents:

Why use PyHeck?
---------------

PyHeck offers `5-10x performance benefits <https://gist.github.com/kevinheavey/eb97728437ebd0fc9bdbd94da61347f5>`_ over the established case conversion library, `inflection <https://github.com/jpvanhal/inflection>`_.

However not all the functions overlap between the two libraries,
and some apparently similar functions may differ in subtle ways. So be careful to check edge cases in whichever library you choose.
