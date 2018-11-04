//I would like to source the html5ever example file for help with this, a good chunk of this code is just implmenting the dom and printing the elments in it and they have a great example on how to do that. I modified it to print the full html document, including closing blocks and opening an html file. I also removed some features that were not needed, and required fixes.

//I have provided a basic input file from from view-source:http://davejohnchapman.com/

//https://github.com/servo/html5ever/blob/master/html5ever/examples/print-rcdom.rs

#[macro_use] extern crate html5ever;

use std::default::Default;
use std::string::String;
use std::io::stdin;


use std::fs::File;
use std::io::prelude::*;

use html5ever::parse_document;
use html5ever::rcdom::{NodeData, RcDom, Handle};
use html5ever::tendril::TendrilSink;
#[allow (unused_variables)]
fn parse(handle: Handle, key: &str) {
    let node = handle;

    print!("");
    match node.data {
        NodeData::Document
        => {},

        NodeData::Doctype { ref name, ref public_id, ref system_id }
        => println!("<!DOCTYPE {}>", name),

        NodeData::Text { ref contents }
        => println!("{}", html_string_sub(&contents.borrow(), key)),

        NodeData::Comment { ref contents }
        => println!("<!-- {} -->", contents),

        NodeData::Element { ref name, ref attrs, .. } => {
            assert!(name.ns == ns!(html));
            print!("<{}", name.local);
            for attr in attrs.borrow().iter() {
                assert!(attr.name.ns == ns!());
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");

        }

        NodeData::ProcessingInstruction { .. } => unreachable!()
    }

    for child in node.children.borrow().iter() {
        parse(child.clone(), key);
    }

    print!("");

    //Print the End Tags
    match node.data {
        NodeData::Element { ref name, ref attrs, .. } => {
            assert!(name.ns == ns!(html));
            print!("</{}", name.local);
            for attr in attrs.borrow().iter() {
                assert!(attr.name.ns == ns!());
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        }
        _ => {}
    }
}

fn read_file() -> String{
    let mut file = File::open("input.html").expect("NO INPUT FILE, TRY ADDING A FILE CALLED 'input.html' IN DIRECTORY WITH PROGRAM");

    let mut string_form = String::new();
    file.read_to_string(&mut string_form).expect("ERROR IN FILE READ");

    return string_form.replace("\n", "");


}

fn html_string_sub(f:&str, key:&str) -> String{


    let mut template: String = String::from("<span style=\"background-color: blue; color: white\">");

    template.push_str(key);
    template.push_str("</span>");
    return f.replace(key, &template[..]);

}

fn main() {
    let data = read_file();

    println!("ENTER KEYWORD: ");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("FAILED TO READ KEYWORD");
    input.pop();
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut data.as_bytes())
        .unwrap();


    parse(dom.document, input.as_str());


}


