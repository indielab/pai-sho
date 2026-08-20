# Goal

The opening of `changes/v0.5.0.md`: a short summary of what breaks, then a
section explaining why the commands changed. A migration table listing every old
command and its replacement follows. The reader used 0.4 and wants to know what
will stop working and why.

Target 200 to 260 words for the section. The summary is separate, two or three
sentences.

# Facts

Only these. Nothing else is known.

These notes are shorthand, not prose. Do not copy the wording. Work out how a
person would say each thing, then write that.

## what changed

- six commands removed: ticket, grant-token, add-peer, pin, remove-peer, surfaces
- surfaces just printed things; its output is part of list now
- the other five were all about connecting two machines
- expose used to run with no arguments. bare `expose 3000` meant every peer the
  daemon currently knew could reach port 3000
- that was a bug, not a default. the design has always been that nothing is
  reachable unless granted
- expose now takes --to <key>, or --all for everyone currently known
- 0.4 scripts break

## how the redesign happened

- the redesign started from the flows, not the code. rule was: write the two
  flows down first, then touch anything
- flows are in docs/scenarios.md. literally the commands a person types, machine
  by machine, in sequence
- flow 1: laptop wants at a shared build box. the laptop already has a daemon up
  serving other peers
- flow 1 did not work. four separate problems:
  - two values had to be couriered around out of band, a key and a token, and
    neither was ever used without the other
  - add-peer accepted a key but had nowhere to put a token
  - the only place a token could go was `pai-sho daemon --enroll`, so a busy
    laptop had to bounce its daemon to admit one peer
  - granting was a separate command run later, on the machine that had started
    the whole thing. skip it and the peer connects fine and sees nothing, which
    looks like a bug
- flow 2 was the laptop-boots-a-VM case. it already worked, so it drove nothing

## naming

- candidates considered and dropped: keeping pin; grant-key; allow paired with
  connect
- objection to all three: they name plumbing, not what the two machines are
  doing to each other
- landed on invite and accept
- careful: pin was a real 0.4 command as well as a rejected candidate name

## what fell out of that pair

- key and token merged into one string, key.code, since they always moved
  together anyway
- granting folded into invite as --expose, since both were done by the same
  person at the same keyboard
- each end labels the other for its own use, with --as

# Constraints

- markdown. summary sentences first, no heading. then a `##` heading, then
  paragraphs
- the heading names what the section answers. not "Where this came from"
- lead the summary with a `Breaking:` label
- no first person. no we, our, I, us. where a sentence needs an actor, name one:
  pai-sho, the redesign, the 0.4 CLI, a person at a keyboard. do not fall back on
  an agentless passive just to dodge "we"
- backticks on command names
- no ADR numbers
- do not say whether the result is better
- the file already has a `# v0.5.0` heading. do not repeat the version
- the expose bug is the item most likely to break someone. it leads the summary,
  and the section has to explain it rather than just announce it. give it real
  space, not one sentence before moving on to the naming history
- the migration table below lists all six old commands and their replacements, so
  the summary does not need to enumerate them again
