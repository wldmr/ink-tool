# The Root Detection algorithm

Illustration of the algorithm that detects stories/story roots.

**GOAL**

- Given a set of input files, find those that represent the roots of stories.

**Input**

- A set of files with their corresponding `INCLUDE` statements
- The `INCLUDE` statements are relative to any *eventual* roots (the ones we’re going to determine)

So if `story_a/main.ink` includes `chapter_1.ink`, that means it refers to a file `story_a/chapter_1.ink`

This means that `INCLUDE`d paths are *relative*, but we don’t know to *what* yet.

**Output**

- A set of “Stories”. Each story has a root file and a set of files that belong to it (its *transitive closure*)

**Algorithm Steps**

0.  Gather input files and detect what they `INCLUDE`. [^1]

1.  Sort candidates by path depth (i.e. the number of parent directories), then lexically by that path, then lexically by file name.
    Each file is a *candidate*

    This ensures that more likely root files are processed first (root files can only import files in or below its own directory).

2.  For each *candidate*, determine the *transitive closure*.

    - Each candidate is part of its own transitive closure; it “imports itself”.

    - For each `INCLUDE` statement, construct a prospective path by joining it with the directory of the root file.

    - If this resolves to an existing file recurse into it, resolving its imports relative to the root.

      - *Repeated/cyclic imports*: for each resolved import, keep track of where it was imported *from* (for later error reporting).
        If we encounter the same target a second time, we *don’t* recurse. This prevents inifite loops.
        “Repeated imports” of the root file are covered by this as well,because of the aforementioned “self import”.

      - *Spurious roots*: If the resolved file is in the list of roots that we’ve found so far, that means the other file isn’t actually a root.
        Remove it from the root list, and “absorb” it into our transitive closure:

        1.  Replace its *self import* by the import location we just imported it from.
        2.  Merge all the *resolved* imports into our own.
        3.  Merge all the *unresolved* imports into our own.

    - If it doesn’t resolve to an existing file, keep track of the failed import statement for error reporting. [^2]

3.  Having absorbed all the *spurious roots* along the way:
    The remaining *candidates* are the final *stories*.

[^1]: These paths are not expected to point to actual files yet; we don’t yet know what they are relative to.

[^2]: Reasons for unresolved files:

    1.  Candidate root isn’t actually a root for that file.
    2.  Candidate is a root for that file, but target doesn’t actually exist (typo, file not yet created).

    We can’t really distinguish between the two at this point:
    The user might be writing a bunch of imports in the main file to plan out the story structure.
    Therefore we can’t optimize this away (an initial thought might be that we cold simply discard candidates whose transitive closure doesn’t resolve to anything.)

## Examples

### Example: Single root, all files exist

0.  Input: Files and their include statements

    ``` ink
    // file: main.ink
    INCLUDE chapter1.ink

    // file: chapter1.ink
    INCLUDE chapter1/beginning.ink
    INCLUDE chapter1/middle.ink
    INCLUDE chapter1/end.ink
    ```

    (plus all the above included files, none of have `INCLUDE` statements)

1.  *Candidates* sorted by depth, then by directory, then by filename:

    | Depth | Directory | Filename      | Included |
    |-------|-----------|---------------|:--------:|
    | 0     |           | chapter1.ink  |    ➖    |
    | 0     |           | main.ink      |    ➖    |
    | 1     | chapter1  | beginning.ink |    ➖    |
    | 1     | chapter1  | end.ink       |    ➖    |
    | 1     | chapter1  | middle.ink    |    ➖    |

    Candidates

2.  Build transitive closure of first file, add it as a root

    | Root         | Includes                                                                                               |
    |--------------|--------------------------------------------------------------------------------------------------------|
    | chapter1.ink | chapter1.ink<sup>self</sup> <br> chapter1/beginning.ink <br> chapter1/end.ink <br> chapter1/middle.ink |

    Roots

    | Depth | Directory | Filename      | Included |
    |-------|-----------|---------------|:--------:|
    | 0     |           | main.ink      |    ➖    |
    | 1     | chapter1  | beginning.ink |    ❎    |
    | 1     | chapter1  | end.ink       |    ❎    |
    | 1     | chapter1  | middle.ink    |    ❎    |

    Candidates: marked all the files that have been included

3.  Same for the second file:

    | Root         | Includes                                                                                               |
    |--------------|--------------------------------------------------------------------------------------------------------|
    | chapter1.ink | chapter1.ink<sup>self</sup> <br> chapter1/beginning.ink <br> chapter1/end.ink <br> chapter1/middle.ink |
    | main.ink     | chapter1.ink                                                                                           |

    Roots after inspecting `main`: Note how we didn’t recurse into `chapter1`, because we’ve already know what its transitive closure was.

4.  Absorb `chapter1.ink` into main:

    | Root     | Includes                                                                                |
    |----------|-----------------------------------------------------------------------------------------|
    | main.ink | chapter1.ink <br> chapter1/beginning.ink <br> chapter1/end.ink <br> chapter1/middle.ink |

    Roots after absorbing `chapter1.ink`: Note the removed •<sup>self</sup> marker from `chapter.ink`:
    The import is ➖ longer an implicit self import, but an actual import statement in `main.ink`.

5.  The remaining candidates are all part of a transitive closure, and can be skipped:

    | Depth | Directory | Filename      | Included |
    |-------|-----------|---------------|:--------:|
    | 1     | chapter1  | beginning.ink |    ❎    |
    | 1     | chapter1  | end.ink       |    ❎    |
    | 1     | chapter1  | middle.ink    |    ❎    |

6.  Done. The only *root* file is `main.ink`

### Example: Multiple roots, all files exist

0.  Input

    File structure:

    ``` plantuml
     @startfiles
    /main.ink
    /demo.ink
    /main/beginning.ink
    /main/end.ink
    /main/middle.ink
    /side/beginning.ink
    /side/end.ink
    /side/middle.ink
    /side/story.ink
     @endfiles
    ```

    With:

    ``` ink
    // file: main.ink
    INCLUDE main/beginning.ink
    INCLUDE main/middle.ink
    INCLUDE main/end.ink
    INCLUDE common/stats.ink

    // file: demo.ink
    INCLUDE main/beginning.ink
    INCLUDE side/beginning.ink
    INCLUDE common/stats.ink

    // file: story.ink
    INCLUDE beginning.ink
    INCLUDE middle.ink
    INCLUDE end.ink
    ```

    (plus all the above included files, none of have `INCLUDE` statements.)

1.  *Candidates*:

    | Depth | Directory | Filename      | Included |
    |-------|-----------|---------------|:--------:|
    | 0     |           | demo.ink      |    ➖    |
    | 0     |           | main.ink      |    ➖    |
    | 1     | common    | stats.ink     |    ➖    |
    | 1     | main      | beginning.ink |    ➖    |
    | 1     | main      | end.ink       |    ➖    |
    | 1     | main      | middle.ink    |    ➖    |
    | 1     | side      | beginning.ink |    ➖    |
    | 1     | side      | end.ink       |    ➖    |
    | 1     | side      | middle.ink    |    ➖    |
    | 1     | side      | story.ink     |    ➖    |

2.  Add `demo.ink` as a root

    | Root     | Includes                                                                                     |
    |----------|----------------------------------------------------------------------------------------------|
    | demo.ink | demo.ink<sup>self</sup><br> main/beginning.ink <br> side/beginning.ink <br> common/stats.ink |

    Roots: `demo.ink` added

    This leaves the remain roots:

    | Depth | Directory | Filename      | Included |
    |-------|-----------|---------------|:--------:|
    | 0     |           | main.ink      |    ➖    |
    | 1     | common    | stats.ink     |    ❎    |
    | 1     | main      | beginning.ink |    ❎    |
    | 1     | main      | end.ink       |    ➖    |
    | 1     | main      | middle.ink    |    ➖    |
    | 1     | side      | beginning.ink |    ❎    |
    | 1     | side      | end.ink       |    ➖    |
    | 1     | side      | middle.ink    |    ➖    |
    | 1     | side      | story.ink     |    ➖    |

3.  Add `main.ink` as a root

    | Root     | Includes                                                                                                    |
    |----------|-------------------------------------------------------------------------------------------------------------|
    | demo.ink | demo.ink<sup>self</sup><br> main/beginning.ink <br> side/beginning.ink <br> common/stats.ink                |
    | main.ink | main.ink<sup>self</sup><br> main/beginning.ink <br> main/middle.ink <br> main/end.ink <br> common/stats.ink |

    Roots: `main.ink` added

    | Depth | Directory | Filename      | Included |
    |-------|-----------|---------------|:--------:|
    | 1     | common    | stats.ink     |    ❎    |
    | 1     | main      | beginning.ink |    ❎    |
    | 1     | main      | end.ink       |    ❎    |
    | 1     | main      | middle.ink    |    ❎    |
    | 1     | side      | beginning.ink |    ❎    |
    | 1     | side      | end.ink       |    ➖    |
    | 1     | side      | middle.ink    |    ➖    |
    | 1     | side      | story.ink     |    ➖    |

    Candidates: `main.ink` removed, included files marked

4.  Skip alread included, add `side/end.ink` as a root

    | Root         | Includes                                                                                                    |
    |--------------|-------------------------------------------------------------------------------------------------------------|
    | demo.ink     | demo.ink<sup>self</sup><br> main/beginning.ink <br> side/beginning.ink <br> common/stats.ink                |
    | main.ink     | main.ink<sup>self</sup><br> main/beginning.ink <br> main/middle.ink <br> main/end.ink <br> common/stats.ink |
    | side/end.ink | side/end.ink<sup>self</sup>                                                                                 |

    Roots: `main.ink` added

    | Depth | Directory | Filename   | Included |
    |-------|-----------|------------|:--------:|
    | 1     | side      | middle.ink |    ➖    |
    | 1     | side      | story.ink  |    ➖    |

    Candidates: `side/end.ink` removed

5.  Add `side/middle.ink` as a root

    | Root            | Includes                                                                                                    |
    |-----------------|-------------------------------------------------------------------------------------------------------------|
    | demo.ink        | demo.ink<sup>self</sup><br> main/beginning.ink <br> side/beginning.ink <br> common/stats.ink                |
    | main.ink        | main.ink<sup>self</sup><br> main/beginning.ink <br> main/middle.ink <br> main/end.ink <br> common/stats.ink |
    | side/end.ink    | side/end.ink<sup>self</sup>                                                                                 |
    | side/middle.ink | side/middle.ink<sup>self</sup>                                                                              |

    | Depth | Directory | Filename  | Included |
    |-------|-----------|-----------|:--------:|
    | 1     | side      | story.ink |    ➖    |

6.  Add `side/story.ink` as a root, absorb `side/end.ink` and `side/middle.ink`

    | Root           | Includes                                                                                                    |
    |----------------|-------------------------------------------------------------------------------------------------------------|
    | demo.ink       | demo.ink<sup>self</sup><br> main/beginning.ink <br> side/beginning.ink <br> common/stats.ink                |
    | main.ink       | main.ink<sup>self</sup><br> main/beginning.ink <br> main/middle.ink <br> main/end.ink <br> common/stats.ink |
    | side/story.ink | side/story.ink<sup>self</sup> <br> side/beginning.ink <br> side/middle.ink <br> side/end.ink                |

7.  No more *candidates*. DONE.

    Note that we have “overlapping” stories here:

    ``` mermaid
    stateDiagram-v2
    demo.ink --> main/beginning.ink
    demo.ink --> side/beginning.ink
    demo.ink --> common/stats.ink

    main.ink --> main/beginning.ink
    main.ink --> main/middle.ink
    main.ink --> main/end.ink
    main.ink --> common/stats.ink

    side/story.ink --> side/beginning.ink
    side/story.ink --> side/middle.ink
    side/story.ink --> side/end.ink
    ```

### Example: Cyclic imports

0.  Input

    ``` ink
    // file: a.ink
    INCLUDE b.ink

    // file: b.ink
    INCLUDE c.ink

    // file: c.ink
    INCLUDE a.ink
    ```

1.  *Candidates*:

    | Depth | Directory | Filename | Included |
    |-------|-----------|----------|:--------:|
    | 0     |           | a.ink    |    ➖    |
    | 0     |           | b.ink    |    ➖    |
    | 0     |           | c.ink    |    ➖    |

2.  First root:

    | Root  | Includes                                       |
    |-------|------------------------------------------------|
    | a.ink | a.ink<sup>self</sup><br> b.ink <br> c.ink <br> |

    This leaves the candidates:

    | Depth | Directory | Filename | Included |
    |-------|-----------|----------|:--------:|
    | 0     |           | b.ink    |    ❎    |
    | 0     |           | c.ink    |    ❎    |

3.  All remaining candidates can be skipped. DONE

## Notes

- `INCLUDE` statements can never refer to something “above” the the root file. That is, paths can’t start with `../`

## Glossary

*path*  
a string of characters that might denote a *file*.

*file*  
an actual file that exists on disk

*story* / *root*  
a file + its *transitive closure* that was determined to be an actual story root (i.e. isn’t imported into anything else)

*self import*  
Each root implicitly imports itself. This is so that the transitive closure contains all files of the story, and root files don’t have to be treated specially regarding repeated/circular imports.

*spurious story* / *spurious root*  
A file (say, `spurious.ink`) that was determined to be a root, but then later found to be imported by another file (`actual.ink`).
In that case, the *spurious root* is “absorbed” by the new root:
It is taken out of the list of roots, and its *self import* is replaced by the import location found in `actual.ink`.

*candidate*  
the same as a *story*, but not yet determined to be a root

*transitive closure*  
the (resolved and unresolved) imports relative to some root.

- *Resolved* imports are those that, after joining the path in the `IMPORT` statement with the directory of the root file, point to existing files in the workspace.

- *Unresolved* imports are those that don’t point to existing files. For these, we keep track of their `IMPORT` statements (for error reporting)
