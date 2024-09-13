# `ink-fmt` – Usage


## Formatting Flow Content (Choices, Gathers, Paragraphs)

### Indentation

```ink input
* hey
* * you
* * * guy
```

becomes

```ink output
* hey
  * * you
      * * * guy
````

This goes for the following paragraphs as well:

```ink input
* choice 1
par 1.1
par 1.2
* * choice 2
par 2.1
par 2.2
```

becomes

```ink output
* choice 1
  par 1.1
  par 1.2
  * * choice 2
      par 2.1
      par 2.2
```

A few more things:
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
* a
  * * a.1
* b
  * * b.1
- c
```


### Spacing of marks

By default, each choice and gather mark is followed by a single space. So

```ink input
**choice
```

and

```ink input
*   *      choice
```

both become:

```ink output
* * choice
```

## Formatting Structure Elements

### Knots and Stitches

Default formatting for both Knots and Stitches:
* no indentation (flush left)
* set apart by a blank line before and after
* marks are separated by one space

Knots:
* opening and closing mark normalized to `===` (ending mark added if necessary)

Knots and stitches are set flush left, along with all top level content inside it.

```ink input
          = stitch_outside_knot
    some text
    == knot
       = stitch_inside_knot
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


## List Definitions

### List Spacing

By default, a list such as

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

## Text

### Whitespace

The presence of whitespace in text content is significant, but the _amount_ isn't.

Therefore, multiple spaces are collapsed into one at the beginning and end of pieces of content
(such as around or inside of conditional text, alternatives, choice-only text, etcs).
If there is no space, then this is significant, and therefore no space is added.

```ink input
+ I was[     afraid.]n't brave.
  "Oh please   {help|   no …      }!" I screamed.
```

```ink output
+ I was[ afraid.]n't brave.
  "Oh please {help| no … }!" I screamed.
```

To be clear, this only applies at the text _boundaries_ (i.e. when text abuts to ink syntax),
but not inside a run of text. That means multiple spaces inside a piece of text are unaffected:

```ink input
I left a    {big|long|huge}-ass         pause
```

becomes

```ink output
I left a {big|long|huge}-ass         pause
```

## Code

* There is one space after `~`
* Binary operators are surrounded by one space, but there is no space between unary operators and their operand
* Function arguments in function calls are normalized the same way as parameter definitions for functions/knots.

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
