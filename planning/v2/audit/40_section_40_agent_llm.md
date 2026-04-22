# Audit Report — §40 Agent / LLM Integration

Audited against corpus as of 2026-04-22.

---

## Absorbed

**Part VII header in spec_new.md (lines 5048-5051).** The Part VII
introduction names "agent/LLM integration" explicitly as one of the
deferred DX surfaces:

> "Part VII names developer-experience surfaces outside the language and
> compiler proper: CLI, dependency management, editor tooling, doc
> generation, agent/LLM integration. Deferred until Parts I-IV lock;
> listed to keep the surfaces from being forgotten."

§40 is consistent with this framing: it is a deferred stub that names the
surfaces without locking any design.

**The three named surfaces in §40** (agent skills for writing/reviewing/
validating `.myco` models; harness support for Myco-aware agents;
conventions for LLMs to reason about the language) are a reasonable and
internally consistent enumeration. No corpus document contradicts or
supersedes any of these three sub-topics.

---

## Superseded

Nothing found. The only corpus hits for "agent/LLM/skill" outside §40
fall into two categories: (a) Claude Code harness UI artifacts captured
in `v2_old/convo_backup.md` (terminal token-count tables, plugin
listings), which are not design content; (b) the word "subagent" used in
chunk reports and `spec_dev_notes.md` to describe the Claude Code
development workflow, not Myco language features. Neither category is
design content that §40 supersedes or is superseded by.

---

## Homeless

**v2_old/open_questions.md line 94 — agent-assisted use as a design
axis.**

> "how much should the package optimize for interactive human use vs
> agent-assisted use?"

This is a design question about the Python-side workflow API ergonomics,
not about §40's named surfaces (skills, harness, LLM conventions). It
belongs to the Python boundary (§23 / §24) rather than §40. The question
is also in a deprecated file (`open_questions_deprecated_use_spec_new.md`
exists and the old file is in `v2_old/`). No action needed for §40.

**v2_old/v2_do_this_first.md line 176 — agent-assisted model adaptation.**

> "less-technical users asking an agent to import and adapt an existing
> model"

This is a long-term vision item about the package system enabling
agent-mediated model reuse. It is closer to §37 (dependency management)
or an unwritten long-term vision section than to §40's narrower scope.
The source file is legacy pre-consolidation material; its content has not
been migrated forward into spec_new.md. §40 does not need to capture
this, but the gap between "conventions so LLMs can reason about the
language" and "LLM-assisted model adaptation as a use case" is worth
noting.

Recommend: if the vision of LLM-assisted model adaptation (import,
adapt, validate) is still alive, add it as a bullet under §40's summary
so the surface is not forgotten. It is distinct from "agent skills for
writing/reviewing" — it implies package interop as a prerequisite.

**riley_project_note.md line 68 — LLMs as a data-scraping tool.**

> "use LLMs to scrape both physiological and community level data on
> trees, build a training run, try to recover a controller"

This is a project-specific workflow item (LLMs as an external data
pipeline tool) rather than a Myco language/DX concern. Per memory note
"Project vs language separation," this does not belong in spec prose at
all. No action for §40.

**spec_dev_notes.md lines 167-168 — testing/property-checking affordances.**

> "Testing / property-checking affordances for `.myco` models. DX-ish.
> Could live in Part VII."

This open item from spec_dev_notes is flagged as potentially belonging to
Part VII but is not currently assigned to any §36-§40 stub. It is not
an agent/LLM concern per se, but it is homeless within Part VII. §40 is
not the right home (it would fit a §41 or under the CLI/tooling sections
more naturally).

Recommend: no action for §40 specifically; flag for §35 (Other Opens) or
as a future Part VII stub.

---

## Conflicts

Nothing found. §40 is a deferred stub; no corpus document commits any
design that contradicts its three named surfaces, and no document locks
or bans any of those surfaces.
