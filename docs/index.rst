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

Python already has a popular case conversion library in `inflection <https://github.com/jpvanhal/inflection>`_.

While ``pyheck`` offers no performance benefits over ``inflection``, not all the functions overlap between the two libraries,
and some apparently similar functions differ in subtle ways. So one reason to use ``pyheck`` is to achieve the same behaviour
as ``heck``.
