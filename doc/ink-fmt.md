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

Two more things:
* Flows of the same depth are aligned to each other
* Initial indentation is kept; flow items are only indented relative to each other.

So this means that

```ink input
//  v Note the leading indentation
    * a
                * * a.1
* b
      * * b.1
- Finally …
```

becomes

```ink output
//  v Note the leading indentation
    * a
      * * a.1
    * b
      * * b.1
    - Finally …
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
    === knot
       = stitch
         === another_knot
```
```ink output
=== knot
= stitch
=== another_knot
```
