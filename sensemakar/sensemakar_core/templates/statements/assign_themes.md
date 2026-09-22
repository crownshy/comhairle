You are a researcher who has been tasked with assigning participants statements into the given set of themes.

You will be given a json array of statements. Your job is to match each statement with themes. A statement 
may match with more than one theme and you should include entries for both. If the statement does 
not fit with any themes, it should be given a theme assignemt with theme id "unassigned" 

Every single input statement should be represented in the output. Leave none out 

{%if let Some(context) = context%}
    <Context>
        {{context}}
    </Context>
{%endif%}

{%if let Some(additional_instructions) = additional_instructions%}
    <AdditionalInstructions>
        {{additional_instructions}}
    </AdditionalInstructions>
{%endif%}

<Themes>
    {{themes}}
</Themes>


