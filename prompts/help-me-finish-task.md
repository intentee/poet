+++
description = ""
title = "Help me with finishing the task"

[arguments.objective]
description = "Describe what you are trying to do"
required = true
title = "Your objective"
+++

**user**: This is what I am trying to achieve with the project: {arguments.objective.input}

{
    let results = search(arguments.objective.input);

    if !results.is_empty() {
        add_text("I think those documents can be helpful");
        add_resource_links(results);
    }
}
