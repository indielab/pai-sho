# Judge cases

A labelled corpus for `JUDGE.md`. Every label came from a person rejecting or
writing the prose, never from a model.

    <case>/sample.md   the prose
    <case>/brief.md    what the writer was given, or a note that there was none
    <case>/expect.md   verdict, what it must catch, and where the label came from
    last-run.txt       the most recent scored run

## Running it

The judge needs a model, so this cannot run in CI. Judge each sample separately,
with no other case in context, then score:

    scripts/judge-eval last-run.txt

## Why both kinds of case

A judge drifts in whichever direction you last pushed it.

**Misses** are the obvious failure: the judge passes something a human rejected,
so it is too soft. Tighten `JUDGE.md`.

**False alarms** are the failure that ruins the tool: the judge blocks prose a
human wrote and was happy with. Tune only against misses and you end up with a
judge nothing can satisfy, which gets ignored, which is worse than no judge.

The should-PASS cases exist to make that impossible to do quietly. They are real
human prose with real quirks: a three-item list, a sentence fragment, stacked
passives, a parenthetical aside, first person. All of it fine. If a `JUDGE.md`
change turns any of them red, the change is wrong even if it fixed a miss.

## Adding a case

Add one whenever a person rejects something the judge passed, or defends
something the judge blocked. Record who said what in `expect.md`, because in six
months the label is only as good as its provenance.
