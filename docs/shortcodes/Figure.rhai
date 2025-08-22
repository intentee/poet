fn do_something() {
   "xd"
}

component Figure(content, alt, path) {
    <script src="resources/ts/controller_foo.tsx" />
    <figure>
        <a href={path} target="_blank">
            <img alt={alt} src={path} />
        </a>
        <figcaption>{alt}</figcaption>
        {do_something()}
        {if path == "xd" { "wow" } else { ":(" }}
    </figure>
}
