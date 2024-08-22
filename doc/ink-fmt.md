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

## Formatting Structure Elements (Knots, Stitches)

Knots and stitches are set flush left, along with all top level content inside it.
They are also set apart by a blank line.

```ink input
          = stitch_outside_knot
    some text
    === knot
       = stitch_inside_knot
         === another_knot
  ~ code
        text
```

```ink output
= stitch_outside_knot

some text

=== knot

= stitch_inside_knot

=== another_knot

~ code
text
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

Likewise, for equals signs in initialized list elements.

```ink input
LIST list = (a=4), (b=8)
```

becomes

```ink output
LIST list = (a = 4), (b = 8)
```

Parentheses are flush against their contents

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
```

