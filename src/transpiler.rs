use super::builder::Builder;
use super::parser::ParseResult;

pub struct KnotsOptions {
    pub summary: bool,
}

/// Transpiles to an HTML page our Knots objects
pub fn transpile(parse_result: ParseResult, options: KnotsOptions) -> String {
    let mut builder = Builder::new();

    builder.orphan_tag("!DOCTYPE html", &[]);
    builder.start_tag("html", &[]);

    builder.start_tag("head", &[]);
    builder.orphan_tag("meta", &[("charset", "utf-8")]);
    builder.orphan_tag(
        "meta",
        &[
            ("name", "viewport"),
            ("content", "width=device-width, initial-scale=1"),
        ],
    );

    builder.start_tag("title", &[]);
    builder.write_content(&parse_result.document_title);
    builder.end_tag(); // </title>

    // normalize css
    builder.start_tag("style", &[]);
    builder.write_content(include_str!("../css/normalize.css"));
    builder.end_tag(); // </style>

    // our own css
    builder.start_tag("style", &[]);
    builder.write_content(include_str!("../css/style.css"));
    builder.end_tag(); // </style>

    builder.end_tag(); // </head>

    builder.start_tag("body", &[]);
    builder.start_tag("header", &[]);

    // document title
    builder.start_tag("p", &[("id", "doctitle")]);
    builder.write_content(&parse_result.document_title);
    builder.end_tag(); // </p>

    // document authors
    if !parse_result.document_authors.is_empty() {
        builder.start_tag("div", &[("class", "docinfo")]);

        // svg author icon
        builder.write_content(include_str!("../icons/profile.svg"));

        let mut authors_buf = parse_result.document_authors[0].to_owned();
        for author in parse_result.document_authors.iter().skip(1) {
            authors_buf = format!("{}, {}", authors_buf, author);
        }
        builder.write_content(&authors_buf);
        builder.end_tag(); // </div>
    }
    builder.end_tag(); // </header>

    // put everything in a flex container
    builder.start_tag("div", &[("class", "flex-container")]);
    builder.start_tag("div", &[("class", "main-content")]);
    builder.start_tag("div", &[("class", "container-lvl1")]);
    builder.write_knots_object(parse_result.root_object);
    builder.end_tag(); // </div> .lvl1-container

    // document license
    if let Some(license) = parse_result.document_license {
        builder.start_tag("div", &[("class", "docinfo discreet"), ("id", "license")]);
        builder.orphan_tag("hr", &[]);
        builder.write_content(include_str!("../icons/ereader.svg"));
        builder.write_content(&format!(
            "This work is available under the {} license",
            license
        ));
        builder.end_tag() // </div>
    }

    builder.end_tag(); // </div> .main-content

    if options.summary && !builder.get_summary().is_empty() {
        builder.start_tag("div", &[("class", "summary-container")]);
        builder.start_tag("div", &[("class", "summary")]);
        builder.inline_tag("p", &[], "Summary");
        builder.start_tag("div", &[("class", "summary-content")]);
        let summary = builder.get_summary().to_vec();

        for item in summary {
            let item_class = format!("lvl{}", item.level);
            builder.inline_tag(
                "a",
                &[
                    ("href", &format!("#{}", item.anchor)),
                    ("class", &item_class),
                ],
                &item.name,
            );
        }

        builder.end_tag(); // </div> .summary-content
        builder.end_tag(); // </div> .summary
        builder.end_tag(); // </div> .summary-container
    }

    builder.end_tag(); // </div> .flex-content

    if builder.should_include_katex {
        builder.start_tag("style", &[]);
        builder.write_content(include_str!("../css/katex.css"));
        builder.end_tag(); // </style>

        builder.start_tag("script", &[]);
        builder.write_content(include_str!("../js/katex.js"));

        builder.write_content(&builder.get_katex_content());
        builder.end_tag(); // </script>
    }

    // if we have code blocks then we should include prism.
    if builder.should_include_prism {
        // style elements are usually added in the head section,
        // but we need to call `builder.write_knots_object()` before,
        // which will determine `builder.should_include_prism`.
        builder.start_tag("style", &[]);
        builder.write_content(include_str!("../css/prism.css"));
        builder.end_tag(); // </style>

        builder.start_tag("script", &[]);
        builder.write_content(include_str!("../js/prism.js"));

        for plugin in builder.get_prism_plugins() {
            builder.write_content(plugin);
        }

        builder.end_tag(); // </script>
    }

    // if we have a diagram then include mermaid
    if builder.should_include_mermaid {
        builder.start_tag("script", &[]);
        builder.write_content(include_str!("../js/mermaid.js"));
        builder.end_tag(); // </script>

        builder.start_tag("script", &[]);
        // set the mermaid theme according to the browser theme
        builder.write_content("mermaid.initialize({theme: window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'base'})");
        builder.end_tag(); // </script>
    }

    builder.end_tag(); // </body>
    builder.end_tag(); // </html>

    builder.into_result()
}
