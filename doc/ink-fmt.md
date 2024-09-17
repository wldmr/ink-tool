# `ink-fmt` – Description of standard formatting

These rules are derived from the style of published Ink content by Inkle
(most notably the [Writing With Ink][] documentation).
However, even these examples are _wildly_ inconsistent in terms of indentation,
space vs. tabs and spacing in general.

Consider this an attempt at distilling the “essence” of that formatting into a set of rules that can be automated.
I expect most people will disagree with at least one of these decisions.
But all in all I hope most people will find them mostly acceptable. 

If you find something particulary unbearable, you can in touch and state your case.

[Writing With Ink]: https://github.com/inkle/ink/blob/master/Documentation/WritingWithInk.md

## Formatting Flow Content (Choices, Gathers, Paragraphs)

* Choices and Gathers are indented to where their parent Flow's content starts.
  The same goes for normal paragraphs
* Marks are separated from each other with a space
* Content is separated by three spaces

```ink input
paragraph 0
* choice 1
paragraph 1
** choice 1.1
paragraph 1.1
*** choice 1.1.1
paragraph 1.1.1
```

becomes

```ink output
paragraph 0
*   choice 1
    paragraph 1
    * *   choice 1.1
          paragraph 1.1
          * * *   choice 1.1.1
                  paragraph 1.1.1
````

* Flows of the same depth are aligned to each other (as you would expect, but it bears saying)
* Initial indentation is removed

So this means that

```ink input
    * a
                * * a.1
  * b
         * * b.1
     - c
```

becomes

```ink output
*   a
    * *   a.1
*   b
    * *   b.1
-   c
```


## Structure Elements

Default formatting for both Knots and Stitches:
* no indentation (flush left)
* set apart by a blank line before and after
* marks are separated by one space
* Knot marks are `===` (three equals signs)

To create a visual hierarchy:
* Knots are offset by 3 blank lines
* Stitches are offset by 2 blank lines
* Paragrphs _may_ be separated by at most one blank line

Knots and stitches are set flush left, along with all top level content inside it.

```ink input
          =stitch_outside_knot
    some text
    ==knot
 =        stitch_inside_knot
         ========= another_knot ==
        more text
```

```ink output
= stitch_outside_knot

some text



=== knot ===


= stitch_inside_knot



=== another_knot ===

more text
```

Heres an example of separating paragraphs: If there is at least one blank line in the input,
there'll be one blank line in the output. If there isn't one, then none is added.

```ink input
It was a dark and stormy night.
To reiterate: It was stormy and also dark.



So anyway, the next day …
```

```ink output
It was a dark and stormy night.
To reiterate: It was stormy and also dark.

So anyway, the next day …
```

The same goes for other sorts of paragraph-level content:

```ink input
VAR did_thing_a = false
VAR did_thing_b = false

VAR has_trinket_a = false
VAR has_trinket_b = false


*   do thing a


    * *   do it this way
    * *   or the other way



*   do thing b
    * *   smoothly
    * *   haphazardly
```

```ink output
VAR did_thing_a = false
VAR did_thing_b = false

VAR has_trinket_a = false
VAR has_trinket_b = false

*   do thing a

    * *   do it this way
    * *   or the other way

*   do thing b
    * *   smoothly
    * *   haphazardly
```

## Functions

* To visually distinguish functons from normal knots, the leading marks are normalized to `==`
  and closing marks (if present) are removed.
* Contents are _not_ set apart by an empty line and are indented to align with the `function` keyword
* Parameter lists don't have a space before
* Commas have no space before and one after

```ink input
=====function    addition ( a ,    b   ) ====
~ return a + b
```

```ink output
== function addition(a, b)
   ~ return a + b
```

Additonally, because functions tend to not be very long (*hem* …), they are separated by only two blank lines.

```ink input
== function neg(x)
   ~ return -x
== function inv(x)
   ~ return 1 / x
```

```ink output
== function neg(x)
   ~ return -x


== function inv(x)
   ~ return 1 / x
```


## List Definitions

### List Spacing

A list such as

```ink input
LIST list=a,b,c
```

becomes

```ink output
LIST list = a, b, c
```

Equals signs in list elements with explicit values are surrounded by spaces, i.e.

```ink input
LIST list = (a=4), (b=8)
```

becomes

```ink output
LIST list = (a = 4), (b = 8)
```

Parentheses are set flush against their contents

```ink input
LIST list = ( a = 2 ), ( b )
```

becomes

```ink output
LIST list = (a = 2), (b)
```

### List Initializer Parentheses Order

List item elements are canonicized to the "parens outside" form.

```ink input
LIST list = (a)=4, (b)=8
```

```ink output
LIST list = (a = 4), (b = 8)
````

## Content / Text

### Whitespace

The presence of whitespace in text content is significant, but the _amount_ isn't.

Therefore, multiple spaces are collapsed into one at the beginning and end of pieces of content
(such as around or inside of conditional text, alternatives, choice-only text, etcs).
If there is no space, then this is significant, and therefore no space is added.

```ink input
+   I was[     afraid.]n't brave.
    "Oh please   {help|   no …      }!" I screamed.
```

```ink output
+   I was[ afraid.]n't brave.
    "Oh please {help| no … }!" I screamed.
```

To be clear, this only applies at the text _boundaries_ (i.e. when text abuts ink syntax),
but not inside a run of text. That means multiple spaces inside a piece of text are unaffected:

```ink input
I left a    {big|long|huge}-ass         pause
```

becomes

```ink output
I left a {big|long|huge}-ass         pause
```

### Multiline logic blocks

#### Multiline Conditionals

Content of multiline conditionals is indented, if the content is not on the same line. Simple if-blocks:

```ink input
{ x > 0:
x is greater than zero
- else:
x is not greater than zero
}
```

```ink output
{ x > 0:
    x is greater than zero
- else:
    x is not greater than zero
}
```

The same goes for extended if blocks:

```ink input
{
- x > 0:
x is greater than zero
- x == 0:
x is zero
- else:
x is smaller than zero
}
```

```ink output
{
- x > 0:
    x is greater than zero
- x == 0:
    x is zero
- else:
    x is smaller than zero
}
```

and switch statements:

```ink input
{ x:
- 0:
x is zero
- 1:
x is one
- else:
x is something else
}
```

```ink output
{ x:
- 0:
    x is zero
- 1:
    x is one
- else:
    x is something else
}
```


If content starts on the same line, then a space is enforced after the ':'.
Both "inline" and "newline" styles can be mixed. For example, here is a simple
if-else block with mixed styles (where content isn't even allowed on the same
line as the first condition; the compiler would reject it):
 
```ink input
{ x > 0:
x is greater than zero
- else:       x is not greater than zero
}
```

```ink output
{ x > 0:
    x is greater than zero
- else: x is not greater than zero
}
```

If there are multiple lines of content after starting on the same line as a condition,
then all subsequent lines are aligned to the content of the first line, like so:

```ink input
{
- long_x > 0: long_x is greater than zero
I think that's a positive.
- else:       long_x is not greater than zero
But that's no cause for negativity.
}
```

```ink output
{
- long_x > 0: long_x is greater than zero
              I think that's a positive.
- else: long_x is not greater than zero
        But that's no cause for negativity.
}
```

#### Multiline Alternatives

* Similar rules apply to multiline alternatives, only that these must start on a new line.
* The keywords are separeed from their surroundings by a single space each

Combined example:

```ink input
{shuffle     once:
   - The coin came up heads.
 Just as I had predicted, with 50% confidence.
  -     It was tails.
     The most surprising thing about that was the number side of a coin is called 'tails'. 
- The damn thing came to rest on its side.
How very strange.
}
```

```ink output
{ shuffle once:
- The coin came up heads.
  Just as I had predicted, with 50% confidence.
- It was tails.
  The most surprising thing about that was the number side of a coin is called 'tails'.
- The damn thing came to rest on its side.
  How very strange.
}
```

## Code

* There is one space after `~`
* Binary operators are surrounded by one space, but there is no space between unary operators and their operand
* Arguments in function/knot/stitch calls are normalized the same way as parameter definitions.

```ink input
~temp sum=a+b
~     temp    neg_ratio   =  - (a  +  b)    /    a
~ temp  result     =addition ( a , b )
```

```ink output
~ temp sum = a + b
~ temp neg_ratio = -(a + b) / a
~ temp result = addition(a, b)
```
