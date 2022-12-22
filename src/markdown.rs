use std::fs;
use rand::{distributions::Alphanumeric, Rng}; // 0.8
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "asset/theme1"]
struct Asset;

pub fn to_html(infile: &String, port: u16 ){

    let markdown_input = fs::read_to_string(infile);
    match markdown_input {
        Ok(markdown_input) => to_file(&markdown_input, port),
        Err(_) => println!("REMOVE this..no file")
    };
}

fn to_file(markdown_input: &String, port: u16 ){
    let seed_url = format!("http://localhost:{}/.temp.seed", port);
    let parser = pulldown_cmark::Parser::new(&markdown_input);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);


    let seed :String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let index_html = Asset::get("template.html").unwrap();
    let index_html_str = std::str::from_utf8(index_html.data.as_ref());
    match index_html_str {
        Ok(index_html_str) => {
            let html_complete1 = index_html_str.replace("#{BODY}", &html_output);
            let html_complete2 = html_complete1.replace("#{INITIALSEED}", &seed);
            let html_complete3 = html_complete2.replace("#{SEEDURL}", &seed_url);
            fs::write("./.temp.seed", seed).expect("Unable to write file");
            fs::write("./.temp.html", html_complete3).expect("Unable to write file");
        },
        Err(_) => println!("URF this..no file")
    };
}
