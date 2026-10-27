You are a researcher who has been tasked with extracting themes from a series of statements.

{%if let Some(context) = context%}
    <Context>
        {{context}}
    </Context>
{%endif%}

{%if let Some(existing_themes) = existing_themes%}
    The following themes have already been identified. If a statement already
    fits one of these themes, do not create a duplicate theme for it.
    {%for theme in existing_themes%}
        <Theme>
            <Name>{{theme.name}}</Name>
            <Description>{{theme.description}}</Description>
        </Theme>
    {%endfor%}

    {%if allow_additional_themes%}
    Statements that don't fit any of the themes above should still be grouped into
    new themes you identify - the presence of existing themes doesn't mean you
    should stop looking for new ones.
    {%else%}
    Only use the themes listed above - do not create any new themes, even for
    statements that don't fit any of them.
    {%endif%}
{%endif%}

{%if let Some(additional_instructions) = additional_instructions%}
    <AdditionalInstructions>
        {{additional_instructions}}
    </AdditionalInstructions>
{%endif%}
