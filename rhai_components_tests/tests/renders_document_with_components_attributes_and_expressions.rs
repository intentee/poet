use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::Func;
use rhai::Map;
use rhai_components_tests::dummy_context::DummyContext;
use rhai_components_tests::fixtures_engine::fixtures_engine;

#[test]
fn renders_document_with_components_attributes_and_expressions() {
    let dummy_context = DummyContext::default();
    let mut props = Map::new();

    props.insert("bar".into(), "baz".into());

    let rendered = Func::<(DummyContext, Dynamic, Dynamic), String>::create_from_script(
        fixtures_engine(),
        r#"
            import "LayoutHomepage" as LayoutHomepage;
            import "Note" as Note;

            fn template(context, props, content) {
                context.assets.add("resources/controller_foo.tsx");

                component {
                    <!DOCTYPE html>
                    <LayoutHomepage extraBodyClass="my-extra-class">
                        < button
                            class="myclass"
                            data-foo={props.bar}
                            data-fooz={`${props.bar}`}
                            data-gooz={if true {
                                component {
                                    <div />
                                }
                            } else {
                                ":)"
                            }}
                            disabled
                        >
                            <b><i><u>test</u></i></b>
                            Hello! :D
                            {" - "}
                            <br />

                            <Note type="warn">
                                {if content.is_empty() {
                                    component {
                                        <div>
                                            NOTE EMPTY CONTENT
                                        </div>
                                    }
                                } else {
                                    content
                                }}
                            </Note>
                        </button>
                    </LayoutHomepage>
                }
            }
        "#,
        "template",
    )
    .map_err(Box::<EvalAltResult>::from)
    .and_then(|renderer| {
        renderer(
            dummy_context.clone(),
            Dynamic::from_map(props),
            Dynamic::from(""),
        )
    });

    assert!(rendered.is_ok_and(|rendered| {
        [
            "<!DOCTYPE html>",
            "<html lang=\"en\">",
            "<title>Poet</title>",
            "<button",
            "class=\"myclass\"",
            "data-foo=\"baz\"",
            "data-fooz=\"baz\"",
            "disabled",
            "<b><i><u>test</u></i></b>",
            "Hello! :D",
            " - ",
            "<br>",
            "class=\"note note--warn\"",
            "NOTE EMPTY CONTENT",
            "</button>",
            "</body>",
            "</html>",
        ]
        .iter()
        .all(|expected_fragment| rendered.contains(expected_fragment))
    }));
    assert!(
        dummy_context
            .assets
            .assets
            .contains("resources/controller_foo.tsx")
    );
}
