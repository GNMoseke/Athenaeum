# Athenaeum
Simple TUI flashcards:
```sh
╔═══════════════BRIDGEKEEPER════════════════╗
║                                           ║
║                                           ║
║                                           ║
║                                           ║
║            WHAT IS YOUR NAME?             ║
║                                           ║
║                                           ║
║                                           ║
║                                           ║
║                                           ║
╚═══════════════════════════════════════════╝

╔═══════════════BRIDGEKEEPER════════════════╗
║                                           ║
║                                           ║
║                                           ║
║                                           ║
║    MY NAME IS SIR LANCELOT OF CAMELOT.    ║
║                                           ║
║                                           ║
║                                           ║
║                                           ║
║                                           ║
╚═══════════════════════════════════════════╝

```
## Installing
Simply clone & run `./install.sh` and check that everything is working with `ath --version`.

## Running
You (currently) must pass the `--sets-dir` and `--set` flag to tell the app where to find flashcard sets and which one you want to run
through. The `--set` name must match the filename (case-insensitive and without the file extension).

Run `ath --help` for all options.

* `Space` flips card
* `N` goes to next card
* `P` goes to previous card

## Making Flashcard Sets
Flashcard sets are simple 2-column `csv` files. See an example in the [example_sets](./example_sets/) directory.
