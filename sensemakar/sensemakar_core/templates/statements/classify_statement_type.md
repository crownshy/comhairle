You are a researcher who has been tasked with classifying participants statements into the given ontology.

You will be given a json array of statements without classification. Your job is to return them with 
statements

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


