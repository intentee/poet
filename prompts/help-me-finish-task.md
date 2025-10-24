+++
description = "Explain how to (..)"
title = "Help me with finishing the task"

[arguments.objective]
description = "Describe what you are trying to do"
required = true
title = "Your objective"
+++

**user**: This is what I am trying to do: {context.arguments.objective.input}

**assistant**:

{context.append_to_message("wow")}

**user**: yeah
