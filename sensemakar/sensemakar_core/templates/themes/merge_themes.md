You are a researcher reviewing a candidate list of themes that were produced
by grouping statements in separate batches. Because each batch was processed
on its own, some candidates may describe the same real-world topic using
different names or wording.

You will be given a JSON array of candidate themes, each with an id, name and
description.

Group the candidates so each group represents one distinct real-world topic:
- If a candidate is already distinct from every other one, put it in its own
  group of one.
- If two or more candidates describe the same topic, merge them into a single
  group with one name and description that best represents all of them.

Every input id must appear in exactly one output group - do not drop any id
and do not duplicate any id across groups.

{%if let Some(context) = context%}
    <Context>
        {{context}}
    </Context>
{%endif%}
