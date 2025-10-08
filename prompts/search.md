+++
description = "Find useful stuff in our docs"
name = "search"
title = "Search the docs"

[[argument]]
description = "Describe what you are looking for"
name = "query"
required = true
title = "Query"
+++
{
    let results = search(arguments.query.content);

    if results.is_empty() {
        component {
            <Message role="assistant">
                <Text>I haven't found anything :( Please read this instead:</Text>
                <Resource uri="poet://content/blog/index.md" />
            </Message>
        }
    } else {
        component {
            <Message role="assistant">
                <Text>I've found some stuff for you</Text>
                <SearchResult results={results} />
            </Message>
        }
    }
}
