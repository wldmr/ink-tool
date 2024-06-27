# `ink-fmt` – Usage

## Formatting Choices and Gathers

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
