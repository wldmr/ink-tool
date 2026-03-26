# The Root Detection algorithm

Illustration of the algorithm that detects stories/story roots.

**GOAL**

- Given a set of input files, find those that represent the roots of stories.

**Input**

- A set of files with their corresponding `INCLUDE` statements
- The `INCLUDE` statements are relative to the *eventual* root (the ones we’re going to determine)

So if `story_a/main.ink` includes `chapter_1.ink`, that means it refers to a file `story_a/chapter_1.ink`

This means that `INCLUDE`d paths are *relative*, but we don’t know to *what* yet.

**Output**

- A set of “Stories”. Each story has a root file and a set of files that belong to it (its *transitive closure*)

**Algorithm Steps**

0.  Gather input files and detect what they `INCLUDE`. [^1]

1.  For each file, determine the *transitive closure*. Each (file, closure) pair is a *candidate*

    - Each candidate is part of its own transitive closure; it “imports itself”.

    - For each `INCLUDE` statement, construct a prospective path by joining it with the directory of the root file.

    - If this resolves to an existing file recurse into it, resolving its imports relative to the root.

      - Repeated imports: for each resolved import, keep track of where it was imported *from* (for later error reporting).
        If we encounter the same target a second time, we don’t recurse. This prevents inifite loops.
      - “Repeated imports” of the root file: If the repeated import is the root file itself, then mark that candiated as *cyclic*.

    - If it doesn’t resolve to an existing file, keep track of the failed import statement for error reporting. [^2]

2.  Prune *candidates*: Keep the candidate only if

    - the path is not included in any other transitive closure, or

    - the candidate is *cyclic* [^3]

3.  The remaining *candidates* are the final *stories*.

[^1]: These paths are not expected to point to actual files yet; we don’t yet know what they are relative to.

[^2]: Reasons for unresolved files:

    1.  Prospective root isn’t actually a root
    2.  File is a root, but target doesn’t actually exist
        (typo, file not yet created)

    We can’t really distinguish between the two at this point:
    The user might be writing a bunch of imports in the main file to plan out the story structure.
    Therefore we can’t optimize this away (an initial thought might be that we cold simply discard candidates whose transitive closure doesn’t resolve to anything.)

[^3]: Cyclic imports have no determinable root, so this treats every cyclic story as its own root.

## Examples

### Example: Single root, all files exist

0.  Input

    ``` yaml
    - main.ink:
      - chapter1.ink
    - chapter1.ink:
      - chapter1/beginning.ink
      - chapter1/middle.ink
      - chapter1/end.ink
    - chapter1/beginning.ink: []
    - chapter1/middle.ink: []
    - chapter1/end.ink: []
    ```

1.  Transitive closures, i.e. *candidates*:

    ``` yaml
    - main.ink:
      - main.ink
      - chapter1.ink
      - chapter1/beginning.ink
      - chapter1/middle.ink
      - chapter1/end.ink
    - chapter1.ink:
      - chapter1.ink
      - chapter1/beginning.ink
      - chapter1/middle.ink
      - chapter1/end.ink
    - chapter1/beginning.ink:
      - chapter1/beginning.ink
    - chapter1/middle.ink:
      - chapter1/middle.ink
    - chapter1/end.ink:
      - chapter1/end.ink
    ```

2.  Detect membership (i.e. “imported by”)

    ``` yaml
    - main.ink: []
    - chapter1.ink: [main.ink]
    - chapter1/beginning.ink: [chapter1.ink]
    - chapter1/middle.ink: [chapter1.ink]
    - chapter1/end.ink: [chapter1.ink]
    ```

3.  Prune. DONE.

    ``` yaml
    - main.ink:
      - chapter1.ink
      - chapter1/beginning.ink
      - chapter1/middle.ink
      - chapter1/end.ink
    ```

### Example: Multiple roots, all files exist

0.  Input

    ``` yaml
    - main.ink:
      - main.ink
      - main/beginning.ink
      - main/middle.ink
      - main/end.ink
      - common/stats.ink
    - side/story.ink:
      - side/story.ink
      - beginning.ink
      - middle.ink
      - end.ink
    - demo.ink:
      - demo.ink
      - main/beginning.ink
      - side/beginning.ink
      - common/stats.ink
    - main/beginning.ink: []
    - main/middle.ink: []
    - main/end.ink: []
    - side/beginning.ink: []
    - side/middle.ink: []
    - side/end.ink: []
    - common/stats.ink: []
    ```

1.  Transitive closures, i.e. *candidates*:

    ``` yaml
    - main.ink:
      - main.ink
      - main/beginning.ink
      - main/middle.ink
      - main/end.ink
      - common/stats.ink
    - side/story.ink:
      # Note how we've resolved the `INCLUDE`d path to actual file paths.
      - side/story.ink
      - side/beginning.ink
      - side/middle.ink
      - side/end.ink
    - demo.ink:
      - main/beginning.ink
      - side/beginning.ink
      - common/stats.ink
    - main/beginning.ink: []
    - main/middle.ink: []
    - main/end.ink: []
    - side/beginning.ink: []
    - side/middle.ink: []
    - side/end.ink: []
    - common/stats.ink: []
    ```

2.  Detect membership

    ``` yaml
    - main.ink: []
    - side/story.ink: []
    - demo.ink: []
    - main/beginning.ink:  [main.ink, demo.ink]
    - main/middle.ink: [main.ink,]
    - main/end.ink: [main.ink,]
    - side/beginning.ink: [main.ink, demo.ink]
    - side/middle.ink: [side.ink,]
    - side/end.ink: [side.ink,]
    - common/stats.ink: [main.ink, demo.ink]
    ```

3.  Prune. DONE.

    ``` yaml
    - main.ink:
      - main.ink
      - main/beginning.ink
      - main/middle.ink
      - main/end.ink
      - common/stats.ink
    - side/story.ink:
      - side/story.ink
      - side/beginning.ink
      - side/middle.ink
      - side/end.ink
    - demo.ink:
      - main/beginning.ink
      - side/beginning.ink
      - common/stats.ink
    ```

### Example: Circular imports

0.  Input

    ``` yaml
    - a.ink:
      - b.ink
    - b.ink:
      - c.ink
    - c.ink:
      - a.ink
    ```

1.  Transitive closures, i.e. *candidates*.
    Repeated imports marked by suffix `×2`:

    ``` yaml
    - a.ink:
      - a.ink×2
      - b.ink
      - c.ink
    - b.ink:
      - a.ink
      - b.ink×2
      - c.ink
    - c.ink:
      - a.ink
      - b.ink
      - c.ink×2
    ```

2.  Detect membership

    ``` yaml
    - a.ink: [a.ink×2, b.ink, c.ink]
    - b.ink: [a.ink, b.ink×2, c.ink]
    - c.ink: [a.ink, b.ink, c.ink×2]
    ```

3.  Prune (or rather, don’t prune in this case). DONE.

    ``` yaml
    - a.ink:
      - a.ink×2
      - b.ink
      - c.ink
    - b.ink:
      - a.ink
      - b.ink×2
      - c.ink
    - c.ink:
      - a.ink
      - b.ink
      - c.ink×2
    ```

## Notes

- `INCLUDE` statements can never refer to something “above” the the root file. That is, paths can’t start with `../`

## Glossary

*path*  
a string of characters that might denote a *file*.

*file*  
an actual file that exists on disk

*story*  
a file + its *transitive closure* that was determined to be an actual story root (i.e. isn’t imported into anything else)

*candidate*  
the same as a *story*, but not yet determined to be a root

*transitive closure*  
the (resolved and unresolved) imports relative to some root.

- *Resolved* imports are those that, after joining the path in the `IMPORT` statement with the directory of the root file, point to existing files in the workspace.
- *Unresolved* imports are those that don’t point to existing files. For these, we keep track of their `IMPORT` statements (for error reporting)
