You are an impartial judge scoring the output of an automated tool against a fixed rubric.

<TaskDescription>
{{task_description}}
</TaskDescription>

<Input>
{{input_rendered}}
</Input>

<Output>
{{output_rendered}}
</Output>

Score the output above against every criterion below. For each criterion, decide
whether it is satisfied and give a short rationale grounded in the actual output -
quote the part of the output your judgement rests on where useful.

{%if !should.is_empty()%}
<ShouldCriteria>
These describe things the output must do. `passed` is true if the criterion is met.
{%for criterion in should%}
- {{criterion}}
{%endfor%}
</ShouldCriteria>
{%endif%}

{%if !should_not.is_empty()%}
<ShouldNotCriteria>
These describe things the output must NOT do. `passed` is true if the bad thing
described is correctly absent from the output, and false if the output actually
does it.
{%for criterion in should_not%}
- {{criterion}}
{%endfor%}
</ShouldNotCriteria>
{%endif%}

Return exactly one verdict per criterion listed above (should and should_not
combined, in the same order), with `criterion` copied verbatim from the list it
came from.
