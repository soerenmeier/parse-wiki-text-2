// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

use crate::test_cases::TEST_CASES;

pub fn run_test(configuration: &parse_wiki_text::Configuration) {
    let mut output = concat!(
        "<title>Parse Wiki Text test cases</title>",
        "<style>",
        "a{color:#006064;display:block;padding:8;text-decoration:none}",
        "a:hover{background:#eee}",
        "body{background:#f7f7f7;display:flex;font-family:sans-serif;height:100%;margin:0}",
        "div div{background:#fff;box-shadow: 0 1px 3px rgba(0,0,0,.12),0 1px 2px rgba(0,0,0,.24);margin:16;padding:16}",
        "h1{font-size:20;margin:24 16 16}",
        "hr{border:0;border-top:1px solid #ccc}",
        "pre{margin:0}",
        "span{color:#aaa}",
        "</style>",
        "<div style=\"background:#fff;box-shadow: 0 1px 3px rgba(0,0,0,.12),0 1px 2px rgba(0,0,0,.24);flex:0 1 220px;overflow:auto\">"
    ).to_owned();
    if let Some(window) = TEST_CASES
        .windows(2)
        .find(|window| window[0].0 >= window[1].0)
    {
        panic!("Sort: {:#?}", (window[0].0, window[1].0));
    }
    for (title, test_cases) in TEST_CASES {
        if let Some(window) = test_cases.windows(2).find(|window| window[0] >= window[1]) {
            panic!("Sort: {:#?}", window);
        }
        output += &format!("<a href=#{}>", title.replace(" ", "_"));
        output += title;
        output += &format!(" <span>{}</span></a>", test_cases.len());
    }
    output += "</div><div style=\"flex:1 1 200px;overflow:auto\">";
    for (title, test_cases) in TEST_CASES {
        output += &format!("<h1 id={}>", title.replace(" ", "_"));
        output += title;
        output += "</h1>";
        for wiki_text in *test_cases {
            output += "<div><pre>";
            output += &wiki_text
                .replace("&", "&amp;")
                .replace("<", "&lt;")
                .replace("\t", "<span>⭾</span>")
                .replace("\n", "<span>⏎</span>\n")
                .replace(" ", "<span>·</span>")
                .replace("</span><span>", "");
            match std::panic::catch_unwind(|| configuration.parse(wiki_text)) {
                Err(_) => {
                    eprintln!("Panic with wiki text {:?}", wiki_text);
                    output += "</pre><hr>panic</div>";
                }
                Ok(result) => {
                    output += "</pre><hr><pre>";
                    output += &format!("{:#?}", result)
                        .replace("&", "&amp;")
                        .replace("<", "&lt;");
                    output += "</pre></div>";
                }
            }
        }
    }
    output += "</div>";
    if let Err(error) = std::fs::write("report.html", output) {
        eprintln!("Failed to write report: {}", error);
        std::process::exit(1);
    }
}
