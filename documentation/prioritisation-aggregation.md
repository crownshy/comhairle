# Aggregating Prioritisation Poll Results

This note describes how the Prioritisation Tool turns a stack of participant
answers into the numbers shown on the "Realtime result" panel. It is written
for the team that will design the production version — not the participants.

## Current model: per-proposal, independent questions

Every proposal owns its own list of questions. There is no shared
question set across proposals, and there is no cross-proposal ranking
in the prototype. The aggregation is therefore strictly per-proposal,
per-question:

- For each `(proposal, question)` pair, count answers (multiple choice),
  compute mean / min / max / variance / 10-bucket histogram (numeric),
  or collect samples (free-text).
- That's it.

This is intentionally minimal. We will revisit the more interesting
question — *which proposals does the room care about most?* — once we
add a shared core of questions on top of the per-proposal extras (see
`documentation/prioritisation-tool-deferred.md` §9).

## Why we are not computing combined metrics yet

The combined "weighted average" / "importance-weighted agreement" /
"variance" formulas only make sense when every proposal is rated on the
**same axes**. With fully per-proposal questions:

- Different proposals can use different question types ("how
  important?" on one, "how much do you agree?" on another, "what
  colour?" on a third).
- Even when two proposals happen to share a "how important?" question,
  there is no way for the system to *know* they are the same axis —
  question ids differ, prompts differ, choice ordering differs.
- Trying to combine them anyway would silently produce nonsense.

So we hold off until the data model can express "this question is the
shared importance axis" explicitly.

## What we *will* compute when the model supports it

When the production version lands a shared-core schema, the formulas
below become meaningful again. They are documented here so the design
isn't lost.

### Notation

For each (participant, proposal) pair, let `imp ∈ [0,1]` be the
normalised importance answer and `agr ∈ [−1,+1]` the normalised
agreement answer.

- Multiple-choice imp/agr questions are mapped by ordinal index 1..N
  then rescaled.
- 5-star questions are 1..5 rescaled.
- Rating-scale questions use `(value − min) / (max − min)`.

Let `n` be the number of submissions that answered both questions for
a proposal. Let `pᵢ = impᵢ × agrᵢ`.

### Formula 1 — Weighted average

```
weightedAverage = (1/n) × Σ pᵢ
```

The mean of the importance-times-agreement product. Range `[−1,+1]`.
Penalises both apathy and disagreement.

### Formula 2 — Importance-weighted agreement

```
importanceWeightedAgreement = Σ(impᵢ × agrᵢ) / Σ impᵢ
```

Average agreement, weighted so voters who said it doesn't matter to
them weigh less. Stays on `[−1,+1]`.

### Formula 3 — Variance (polarisation)

```
variance = (1/n) × Σ (pᵢ − weightedAverage)²
```

A high variance with a near-zero mean is the signature of a polarising
proposal.

### Formula 4 — Bayesian dampening (deferred)

```
posterior = (n × weightedAverage + k × globalMean) / (n + k)
```

For small rooms, shrink toward the global prior. Pick `k` ≈ 5 for
small groups. Not implemented; show plain `n` next to scores for now.

## A worked example (for reference)

Three voters answer a proposal with shared importance + agreement:

| Voter | imp (raw / norm)         | agr (raw / norm) | p = imp × agr |
|-------|---------------------------|-------------------|---------------|
| A     | "Crucial" (4/4=1.0)       | 9/10 → +0.8       | +0.80         |
| B     | "Important" (3/4=0.75)    | 4/10 → −0.2       | −0.15         |
| C     | "Marginal" (2/4=0.5)      | 0/10 → −1.0       | −0.50         |

- `weightedAverage = (+0.80 − 0.15 − 0.50) / 3 = +0.05`
- `Σ imp = 2.25`, `Σ p = +0.15`, so `importanceWeightedAgreement = +0.067`
- Mean imp = 0.75; mean agr = −0.13; variance over `pᵢ` ≈ 0.30.

The room cares (high mean importance) but is split (low mean
agreement, high variance). Combined "weighted average" comes out near
zero — the honest answer.

## Open questions for the production design

- **Non-respondents**: silence as "low importance" (count as imp=0) vs
  "missing data" (exclude from average)? The prototype excludes.
- **Outliers and gaming**: variance exposes them but doesn't fix them.
  Bayesian dampening / trimmed means are the next step.
- **Direct ranking**: drag-and-drop ordering is a separate scoring
  problem (Borda, Kemeny-Young, Schulze). Treat as a new poll mode,
  not a new question type.
- **Per-proposal question overrides on top of a shared core**: the data
  model needs a join between `proposal_id` and `question_id` so the
  shared core questions are reused across proposals while extras stay
  attached to their proposal.
