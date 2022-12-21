use criterion::*;

const INPUT: &str = r#"
<!doctype html>
<html>
<head>
    <title>Example Domain</title>
    <meta charset="utf-8" />
    <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
</head>
<body>
<div>
    <h1>Example Domain</h1>
    <p>This domain is for use in illustrative examples in documents. You may use this
    domain in literature without prior coordination or asking for permission.</p>
    <p><a href="https://www.iana.org/domains/example">More information...</a></p>
</div>
</body>
</html>
"#;

pub fn criterion_benchmark(cr: &mut Criterion) {
    cr.bench_function("tl", |b| {
        b.iter(|| {
            let dom = tl::parse(black_box(INPUT), tl::ParserOptions::default()).unwrap();
            let parser = dom.parser();
            let title = dom
                .query_selector("title")
                .and_then(|mut iter| iter.next())
                .and_then(|node_handle| node_handle.get(parser))
                .and_then(|node| Some(node.inner_text(parser).to_string()))
                .unwrap();
            assert_eq!("Example Domain", title);
        });
    });
    cr.bench_function("scraper", |b| {
        b.iter(|| {
            let document = scraper::Html::parse_document(black_box(INPUT));
            let selector = scraper::Selector::parse(r#"title"#).unwrap();
            let title = document.select(&selector).next().unwrap();
            assert_eq!("Example Domain", title.inner_html());
        });
    });
    cr.bench_function("kuchiki", |b| {
        b.iter(|| {
            let document = kuchiki::traits::TendrilSink::one(kuchiki::parse_html(), INPUT);
            for title_match in document.select("title").unwrap() {
                let as_node = title_match.as_node();
                let text_node = as_node.first_child().unwrap();
                let title = text_node.as_text().unwrap().borrow();
                assert_eq!("Example Domain", title.to_string());
            }
        });
    });
    cr.bench_function("visdom", |b| {
        b.iter(|| {
            let root = visdom::Vis::load(black_box(INPUT)).unwrap();
            let element = root.find("title");
            let title = element.text();
            assert_eq!("Example Domain", title);
        });
    });
    cr.bench_function("html_editor", |b| {
        b.iter(|| {
            use html_editor::operation::Queryable;
            let document = html_editor::parse(black_box(INPUT)).unwrap();
            let selector = html_editor::operation::Selector::from("title");
            let element = document.query(&selector).unwrap();
            let title = html_editor::operation::Htmlifiable::html(&element);
            assert_eq!("<title>Example Domain</title>", title);
        });
    });
    cr.bench_function("lol_html", |b| {
        b.iter(|| {
            let element_handler = |el: &mut lol_html::html_content::Element| {
                if !el.tag_name().eq("title") {
                    el.remove();
                }
                Ok(())
            };
            let comment_handler = |c: &mut lol_html::html_content::Comment| {
                c.remove();
                Ok(())
            };
            let _output = lol_html::rewrite_str(
                black_box(INPUT),
                lol_html::RewriteStrSettings {
                    element_content_handlers: vec![
                        lol_html::element!("*", element_handler),
                        lol_html::comments!("*", comment_handler),
                    ],
                    ..lol_html::RewriteStrSettings::default()
                },
            )
            .unwrap();
            assert!(true)
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
