Sandalwood: a unit-testing language
===================================

I've written unit tests in PyUnit, JUnit, RSpec, QUnit, and bats (as
well as rust's built-in testing macros), and I've noticed that the
better I write my tests, the less they look like the language they're
written in.

As I refactor my tests, they start looking like something else entirely.
In fact, they start looking like the same kind of something.

So I'm working on creating a unit-testing language.

Why not a framework?
--------------------

I don't want to marry any particular programming language. I want the
particular language to be a module. For example, if you're testing
python code, then your sandalwood tests would go in `.py.sn` test files
that contain python code but are wrapped in sandalwood code.

Why? Because our understanding of what constitutes "good tests" keeps
evolving. Why should some languages have up-to-date frameworks while
others do not? I like the idea of a language that can compile to
different languages. That way, updates to our understanding of "good
tests" can be applied in one place, separate from the languages
themselves.

Why not Cucumber/FitNesse/\[name a thing\]?
-------------------------------------------

Because unit tests are fundamentally different from behavioral tests. I
quite like the syntax of Gherkin (the language used by Cucumber); here's
an example in case you're unfamiliar:

```gherkin
Scenario: User logs in

  Given I am on the homepage
  When I click on the "Login" button
   And I fill in the "Email" field with "eddie@email.com"
   And I fill in the "Password" field with "mysecurepassword"
   And I click on "Submit"
  Then I should see "Welcome to this website, Eddie Dean"
```

This is nice for several reasons:

1.  It is very readable. I would like unit tests to be this readable.
2.  It is so separated from the code that nonprogrammers can write the
    tests. This is great for behavioral tests but absurd for unit tests.

A behavioral test ensures that a system behaves the way a client wants
it to behave. The client doesn't (or at least shouldn't) care how it's
done, so their tests should specify that as little as possible.

A unit test tests specific modules, classes, and methods. A unit test is
written and read by programmers. Therefore a unit test *should contain
code.*

Writing unit tests should be easy
---------------------------------

Very few testing frameworks make it easy to write tests. You need not
only to remember which modules you need but also to figure out how to
force your particular language to form itself around tests.

Writing unit tests should be simple. You should be able to import the
code you're testing using the language it's in, but then you should
write the tests themselves with as little overhead as possible.

A new language is a dangerous solution to this problem, as a new
language necessarily adds complexity. Therefore, one of my main goals is
to write a language that can be used to quickly write (gasp) ugly tests.

What will sandalwood look like?
-------------------------------

I don't know. As I said, I like gherkin, so it's a good place to start.

I've been doing a lot of python programming recently, so here is an
example in python:

```python
from hamcrest import *
from unittest import TestCase

from .local_lib import MagicClass

class GivenDefaultInit (TestCase):

    def setUp (self):
        # Our "given" code looks like a test.
        self.magic_obj = MagicClass()

    # We have to write function names in this weird mix of camelCase and
    # underscores to simulate a real sentence.
    def test_whenFrogblasting5_magicClassReturns12 (self):
        frogblast = self.magic_obj.frogblast(5)
        assert_that(frogblast, is_(equal_to(12)))

    def test_whenBibbityBobbity_magicClassReturnsNone (self):
        bibbity_bobbity = self.magic_obj.bibbity_bobbity()
        assert_that(bibbity_bobbity, is_(none()))

class GivenDefaultInitAndModifier4 (GivenDefaultInit):

    def setUp (self):
        # We have to call the superclass function, reminding us that
        # we're dealing with classes and not nested contexts. RSpec
        # handles this well, but I can't use it in python.
        super().setUp()
        self.magic_obj.set_modifier(4)

    def test_whenFrogblasting5_magicClassReturns50 (self):
        frogblast = self.magic_obj.frogblast(5)
        assert_that(frogblast, is_(equal_to(50)))
```

I would like this to instead look like this:

```
environment

    from .local_lib import MagicClass

context default init

    magic_obj = MagicClass()

given default init

when frogblast = magic_obj.frogblast(5)
then frogblast == 12

when bibbity_bobbity = magic_obj.bibbity_bobbity()
then bibbity_bobbity is None

given default init
and magic_obj.set_modifier(4)
when frogblast = magic_obj.frogblast(5)
then frogblast == 50
```

In the end, I don't expect sandalwood to look like this. At the moment,
I don't have specific expectations. I am now starting to rewrite unit
test files I have into files that make sense to me.

GNU General Public License v3 (GPLv3)
-------------------------------------

Sandalwood is a GPLv3 project. If you don't know what it means, it means
that you can copy, distribute, and modify anything and everything here,
but you have to preserve headers and licensing. Pretty standard stuff.

What sets this license apart from others is that you *must* release your
software under the GPLv3 if it contains modified versions of this code
or if it includes this code (e.g. via compilation).

For more information written in legal, see [COPYING](COPYING).
