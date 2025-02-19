# A formal proof assistant for first order logic in the terminal

```
                                                ──────────────h
                                                ~(P∨~P), P ⊢ P
                      ────────────────────h    ─────────────────∨i,l
                      ~(P∨~P), P ⊢ ~(P∨~P)     ~(P∨~P), P ⊢ P∨~P
                      ──────────────────────────────────────────mp
                                    ~(P∨~P), P ⊢ ⊥
                                    ──────────────➔i
                                     ~(P∨~P) ⊢ ~P
─────────────────h                  ──────────────∨i,r
~(P∨~P) ⊢ ~(P∨~P)                   ~(P∨~P) ⊢ P∨~P
──────────────────────────────────────────────────mp
                    ~(P∨~P) ⊢ ⊥
                    ───────────➔i
                    ⊢ ~~(P∨~P)
```

This is a read-eval-print-loop to prove a formula in first order logic.

It reads the formulas from the `sequents.txt` file, one by line. It uses a custom notation to write formulas. For instance:

```
P \/ Q -> ~P -> Q
```

Which is a succinct notation for the more verbose s-expression:

```
(imply (and "P" "Q") (imply (not "P") "Q"))
```

In the REPL, the following commands are accepted.

```
COMMANDS:
  :b            back one step, undo the last action
  :r            reset all steps, undo all actions
  :h            print this help message
  :q            quit the program

APPLICABLE RULES:
  h             hypothesis
  i             introduction of the conclusion (automatic: it choses introduction rule base on conclusion type)
  xf            exflaso
  e <N>         elimination of the Nth hypothesis (automatic: it choses elimination rule base on hypothesis type)
  ii            implication introduction
  iis           implications introduction (for chaining implications)
  dil           disjonction introduction left
  dir           disjonction introduction right
  mp <F>        modus ponens on F (a logical property formula like: ~P/\Q)
  de <F>, <F>   disjonction elimination of left formula and right formula
  ce <F>, <F>   conjonction elimination of left formula and right formula
```
