from typing import Sequence

def snake(s: str) -> str:
    """Convert to snake_case.

    In snake_case, word boundaries are indicated by underscores.

    Example:
        >>> from pyheck import snake
        >>> snake("We carry a new world here, in our hearts.")
        'we_carry_a_new_world_here_in_our_hearts'
    """

def snake_many(strings: Sequence[str]) -> list[str]:
    """Convert a list of strings to snake_case.

    In snake_case, word boundaries are indicated by underscores.

    Example:
        >>> from pyheck import snake_many
        >>> snake_many(["DeviceType", "fooBar"])
        ['device_type', 'foo_bar']
    """

def lower_camel(s: str) -> str:
    """Convert to lowerCamelCase.

    In lowerCamelCase, word boundaries are indicated by capital letters,
    excepting the first word.

    Example:
        >>> from pyheck import lower_camel
        >>> lower_camel("It is we who built these palaces and cities.")
        'itIsWeWhoBuiltThesePalacesAndCities'
    """

def lower_camel_many(strings: Sequence[str]) -> list[str]:
    """Convert a list of strings to lowerCamelCase.

    In lowerCamelCase, word boundaries are indicated by capital letters,
    excepting the first word.

    Example:
        >>> from pyheck import lower_camel_many
        >>> lower_camel_many(["It is we", "who built these"])
        ['itIsWe', 'whoBuiltThese']
    """

def title(s: str) -> str:
    """Convert to Title Case.

    In Title Case, word boundaries are indicated by spaces, and every word is
    capitalized.

    Example:
        >>> from pyheck import title
        >>> title("We have always lived in slums and holes in the wall.")
        'We Have Always Lived In Slums And Holes In The Wall'
    """

def title_many(strings: Sequence[str]) -> list[str]:
    """Convert a list of strings to Title Case.

    In Title Case, word boundaries are indicated by spaces, and every word is
    capitalized.

    Example:
        >>> from pyheck import title_many
        >>> title_many(["We have always", "lived in slums"])
        ['We Have Always', 'Lived In Slums']
    >>> print(title_many.__doc__)
    """

def upper_camel(s: str) -> str:
    """Convert to UpperCamelCase.

    In UpperCamelCase, word boundaries are indicated by capital letters,
    including the first word.

    Example:
        >>> from pyheck import upper_camel
        >>> upper_camel("We are not in the least afraid of ruins.")
        'WeAreNotInTheLeastAfraidOfRuins'
    """

def upper_camel_many(strings: Sequence[str]) -> list[str]:
    """Convert a list of strings to UpperCamelCase.

    In UpperCamelCase, word boundaries are indicated by capital letters,
    including the first word.

    Example:
        >>> from pyheck import upper_camel_many
        >>> upper_camel_many(["We are not", "in the least"])
        ['WeAreNot', 'InTheLeast']
    """

def kebab(s: str) -> str:
    """Convert to kebab-case.

    In kebab-case, word boundaries are indicated by hyphens.

    Example:
        >>> from pyheck import kebab
        >>> kebab("We are going to inherit the earth.")
        'we-are-going-to-inherit-the-earth'
    """

def kebab_many(strings: Sequence[str]) -> list[str]:
    """Convert list of strings to kebab-case.

    In kebab-case, word boundaries are indicated by hyphens.

    Example:
        >>> from pyheck import kebab_many
        >>> kebab_many(["We are going", "to inherit the earth."])
        ['we-are-going', 'to-inherit-the-earth']
    """

def shouty_kebab(s: str) -> str:
    """Convert to SHOUTY-KEBAB-CASE.

    In SHOUTY-KEBAB-CASE, word boundaries are indicated by hyphens and all
    words are in uppercase.

    Example:
        >>> from pyheck import shouty_kebab
        >>> shouty_kebab("We are going to inherit the earth.")
        'WE-ARE-GOING-TO-INHERIT-THE-EARTH'
    """

def shouty_kebab_many(strings: Sequence[str]) -> list[str]:
    """Convert a list of strings to SHOUTY-KEBAB-CASE.

    In SHOUTY-KEBAB-CASE, word boundaries are indicated by hyphens and all
    words are in uppercase.

    Example:
        >>> from pyheck import shouty_kebab_many
        >>> shouty_kebab_many(["We are going", "to inherit the earth."])
        ['WE-ARE-GOING', 'TO-INHERIT-THE-EARTH']
    """

def shouty_snake(s: str) -> str:
    """Convert to SHOUTY_SNAKE_CASE.

    In SHOUTY_SNAKE_CASE, word boundaries are indicated by underscores and all
    words are in uppercase.

    Example:
        >>> from pyheck import shouty_snake
        >>> shouty_snake("That world is growing in this minute.")
        'THAT_WORLD_IS_GROWING_IN_THIS_MINUTE'
    """

def shouty_snake_many(strings: Sequence[str]) -> list[str]:
    """Convert a list of strings to SHOUTY_SNAKE_CASE.

    In SHOUTY_SNAKE_CASE, word boundaries are indicated by underscores and all
    words are in uppercase.

    Example:
        >>> from pyheck import shouty_snake_many
        >>> shouty_snake_many(["That world is", "growing in this minute."])
        ['THAT_WORLD_IS', 'GROWING_IN_THIS_MINUTE']
    """
