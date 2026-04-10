# Ink Examples

The files is this directory serve two purposes:

1.  To illustrate (and test) how the Language Server treats ink files.
    These files will often contain comments like

    ``` ink
    VAR name = "Inigo Montoya"
    //  ^^^^ defines name
    ```

    These are there to drive tests, for example to show what links where, and which parts of the file would show errors when opened in an LSP-enabled editor.

2.  Secondly, some files may containt `ink-test` comments:

    ``` ink
    VAR name = "Inigo Montoya"
    Hello. My name is {name}.

    /* TEST
    Hello. My name is Inigo Montoya.
    */
    ```

    These are tests against `inklecate`, not the LSP.
    By colocating these these two kinds of tests in the same files,
    we want to ensure that the LSP behaves consistent with the reference implementation.
