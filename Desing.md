# Techinal Design Document
This document describes what `brewmaster` should be able to do.

`brewmaster` is a CLI program which aims to help users manage their KEG directories. It achievies this using `git` and `git` hooks.

Once the KEG directory is created the user should make the directory as a `git` tracked directory by issuing a `git init` command.
Then `brewmaster` will be able to help the user to manage the KEG directory.


## Vision
When I - as the end user who wish to manage a KEG directory - create a directory inside a KEG directory certain things __has__ to happen to the KEG directory in order it to be considered to be a valid KEG directory.

> From now on I will refer to these sub directories as KEG Nodes or simply Node(s). In order to be considered as a valid Keg Node a Node has to have specific files. These specific files are [listed]() in the KEG specification.

1. The files inside the newly created has match the specification
2.



## Technical Specificaiton

When the User modifies files in a KEG directory `brewmaster` will pick up these changes before the commit.
The output of `git status --porcelain` will be passed as the argument to `brewmaster`.
`brewmaster` will analyze these changes and will decide how and what should be updated in the KEGNODES file located at the root of the KEG directory.

Git will report certain actions that could happen to a file:
- A -> added a new file previously not tracked by git
- D -> deleted a file
- M -> modified a file that was already tracked by git
- R -> rename of a file which was tracked
- ?? -> untracked file (will not be the part of a commit)

For each of these actions `brewmaster` should handle the KEGNODES modifiaction differently.


__Note__:
> The `isosec` identifier should be the same for all actions that are being made for the commit.
> That means of 1 `isosec` value for a commit, not more.

### Adding a new file to a KEG Node
When the action is `A - add`, `brewmaster` should do the following:
- check whether the slug (unique name of the KEG Node - the directory name) is already present in the KEGNODES files
    1. If it is not, then a new entry should be appended to the KEGNODES file in a format `{isosec} {slug} {node-type}`
    2. If it is present, then the entry should be deleted from the KEGNODES file and `brewmaster` should re-check the `node-type` of the Node and insert a new entry with the same `slug`, the `node-type` (could be different than the deleted entry) and a newly calculated `isosec`.

### Deleting a file from a KEG Node
A couple of things have to happen when the action is `D - delete`:
1. `brewmaster` should check if the deletion of the file results to the whole KEG Node being deleted:
    1. If true, then the entry for the `slug` should be deleted from KEGNODES
    2. If it does not, then `brewmaster` should re-check the `node-type` and update the KEGNODES file by doing:
        - Deleting the previous entry
        - Adding a new entry with a new `isosec` same `slug` and the `node-type`
