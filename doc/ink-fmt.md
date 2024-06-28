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

Knots and stitches are flush left

```ink input
          = stitch_outside_knot
    === knot
       = stitch_inside_knot
         === another_knot
```
```ink output
= stitch_outside_knot
=== knot
= stitch_inside_knot
=== another_knot
```
