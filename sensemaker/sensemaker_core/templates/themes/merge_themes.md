You are a researcher who has been tasked with extracting themes from a series of statements.

{%if let Some(context) = context%}
    <Context>
        {{context}}
    </Context>
{%endif%}

{%if let Some(existing_themes) = existing_themes%}
    The following themes have already been identified. If a statement already 
    fits one of these themes ignore it.
    {%for theme in existing_themes%}
        <Theme>
            <Name>{{theme.name}}</Name>
            <Name>{{theme.description}}</Name>
        </Theme>
    {%endfor%}
{%endif%}

{%if let Some(additional_instructions) = additional_instructions%}
    <AdditionalInstructions>
        {{additional_instructions}}
    </AdditionalInstructions>
{%endif%}
