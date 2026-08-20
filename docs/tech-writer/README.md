# Tech writer

Prose for this repo (changelogs, README sections, release notes) is drafted by a
subagent given `PROMPT.md`, a brief, and nothing else. It has no exposure to the
surrounding conversation, so voice drift in a long session does not leak into the
page.

## The loop

1. Write a brief: the goal, the facts, and the constraints. Facts only. The
   writer is told to invent nothing, so anything missing from the brief cannot
   appear in the output.
2. Run the writer with `PROMPT.md` plus the brief.
3. Run `scripts/prose-check` on the draft.
4. On failure, **change `PROMPT.md`, not the draft**, and go back to 2. Fixing
   the draft by hand teaches nothing and the next page repeats the mistake.
5. When it passes and the facts check out, save the draft as `accepted.md`.

The aim is the smallest prompt that reliably passes. A rule that never fires is
noise; delete it.

## Layout

    PROMPT.md              the writer's instructions
    PROMPT.sha256          hash of PROMPT.md when cases were last regenerated
    cases/<name>/brief.md      goal, facts, constraints
    cases/<name>/accepted.md   the output that was accepted

## Regression testing

Cases are a corpus, not a unit test. The writer is not deterministic, so an
identical rerun is not the bar. What a rerun checks is whether a prompt change
broke a page that used to come out clean.

Two layers, because the model needs an API key and CI does not have one:

`prose-check` reads `git ls-files`, so stage your changes before running it or
new files are skipped. CI does not have that problem.

**In CI, no key.** `scripts/prose-check` runs over every tracked markdown file,
including every `accepted.md`. This gates what ships. It also fails when
`PROMPT.sha256` does not match `PROMPT.md`, which means the prompt was edited
without rerunning the cases.

**Locally, with a key.** After editing `PROMPT.md`, rerun every brief, check the
drafts, and update `accepted.md` and `PROMPT.sha256`:

    sha256sum docs/tech-writer/PROMPT.md | cut -d' ' -f1 > docs/tech-writer/PROMPT.sha256

## Adding a case

Add one whenever a page needed more than one round to get right. The failure is
the useful part: it is evidence of a rule the prompt is missing.
