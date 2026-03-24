# The Root Detection algorithm

Illustration of the algorithm that detects stories/story roots.

**GOAL**

-   Given a set of input files, find those that represent the roots of stories.

**Input**

-   A set of files with their corresponding `INCLUDE` statements
-   The `INCLUDE` statements are relative to the *eventual* root (the ones we're going to determine)

So if `story_a/main.ink` includes `chapter_1.ink`, that means it refers to a file `story_a/chapter_1.ink`

This means that `INCLUDE`d paths are *relative*, but we don't know to *what* yet.

**Output**

-   A set of "Stories". Each story has a root file and a set of files that belong to it (its *transitive closure*)

**Algorithm Steps**

0.  Gather input files and detect what they `INCLUDE`. [^1]

1.  For each file, determine the *transitive closure*. Each (file, closure) pair is a *candidate*

    -   TODO: What to do about files that don't exist?
        -   Reason 1: File isn't actually a root
        -   Reason 2: File is a root, but target doesn't actually exist
            (typo, file not yet created)

2.  For each *candidate*: Remove if path is included in any other transitive closure.

3.  The remaining *candidates* are the final *stories*

[^1]: These paths are not expected to point to actual files yet; we don't yet know what the are relative to

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
      - chapter1.ink
      - chapter1/beginning.ink
      - chapter1/middle.ink
      - chapter1/end.ink
    - chapter1.ink:
      - chapter1/beginning.ink
      - chapter1/middle.ink
      - chapter1/end.ink
    - chapter1/beginning.ink: []
    - chapter1/middle.ink: []
    - chapter1/end.ink: []
    ```

2.  Detect membership

    ``` yaml
    - main.ink:
      - chapter1.ink
      - chapter1/beginning.ink
      - chapter1/middle.ink
      - chapter1/end.ink
    - chapter1.ink: # part of main.ink
      - chapter1/beginning.ink
      - chapter1/middle.ink
      - chapter1/end.ink
    - chapter1/beginning.ink: [] # part of chapter.ink
    - chapter1/middle.ink: [] # part of chapter.ink
    - chapter1/end.ink: [] # part of chapter.ink
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
      - main/beginning.ink
      - main/middle.ink
      - main/end.ink
      - common/stats.ink
    - side/story.ink:
      - beginning.ink
      - middle.ink
      - end.ink
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

1.  Transitive closures, i.e. *candidates*:

    ``` yaml
    - main.ink:
      - main/beginning.ink
      - main/middle.ink
      - main/end.ink
      - common/stats.ink
    - side/story.ink:
      # Note how we've resolved the `INCLUDE`d path to actual file paths.
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
    - main.ink:
      - main/beginning.ink
      - main/middle.ink
      - main/end.ink
      - common/stats.ink
    - side/story.ink:
      - side/beginning.ink
      - side/middle.ink
      - side/end.ink
    - demo.ink:
      - main/beginning.ink
      - side/beginning.ink
      - common/stats.ink
    - main/beginning.ink: [] # included in main.ink & demo.ink
    - main/middle.ink: [] # included in main.ink
    - main/end.ink: [] # included in main.ink
    - side/beginning.ink: [] # included in main.ink & demo.ink
    - side/middle.ink: [] # included in side.ink
    - side/end.ink: [] # included in side.ink
    - common/stats.ink: [] # included in main.ink & demo.ink
    ```

3.  Prune. DONE.

    ``` yaml
    - main.ink:
      - main/beginning.ink
      - main/middle.ink
      - main/end.ink
      - common/stats.ink
    - side/story.ink:
      - side/beginning.ink
      - side/middle.ink
      - side/end.ink
    - demo.ink:
      - main/beginning.ink
      - side/beginning.ink
      - common/stats.ink
    ```

## Notes

-   `INCLUDE` statements can never refer to something "above" the the root file. That is, paths can't start with `../`

## Glossary

*path*\
a string of characters that might denote a *file*.

*file*\
an actual file that exists on disk

*story*\
a file + its *transitive closure* that was determined to be an actual story root (i.e. isn't imported into anything else)

*candidate*\
the same as a *story*, but not yet determined to be a root

*transitive closure*\
a set of paths
