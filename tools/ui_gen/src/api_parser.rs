use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use pest::prelude::*;

impl_rdp! {
    grammar! {
        chunk = { block ~ eoi }
        block = { stat* }

        stat = {
            whitespace |
            comment |
            tableconstructor
        }

        tableconstructor =  { name ~ ["{"] ~ fieldlist? ~ ["}"] }
        fieldlist        =  { field ~ (fieldsep ~ field)* ~ fieldsep* }
        field            =  { var | function }
        fieldsep         = _{ [","] }

        ftype            = { name }
        rettype          = { name }
        callback         = { ["[callback]"] }
        retexp           = { ["->"] ~ name }
        var              = { ftype ~ name }
        varlist          = { var ~ ([","] ~ var)* }
        function         = { callback? ~ name ~ ["("] ~ varlist? ~ [")"] ~ retexp? }

        name = @{
            (['a'..'z'] | ['A'..'Z'] | ["_"]) ~ (['a'..'z'] | ['A'..'Z'] | ["_"] | ['0'..'9'])*
        }

       comment = _{
            ["//"] ~ (!(["\r"] | ["\n"]) ~ any)* ~ (["\n"] | ["\r\n"] | ["\r"] | eoi)
        }

        whitespace = _{ [" "] | ["\t"] | ["\u{000C}"] | ["\r"] | ["\n"] }
    }
}

#[derive(Debug)]
pub struct Variable<'a> {
    pub name: &'a str,
    pub vtype: &'a str,
}

#[derive(Debug)]
pub struct Function<'a> {
    pub name: &'a str,
    pub function_args: Vec<Variable<'a>>,
    pub return_val: Option<Variable<'a>>,
}

#[derive(Debug)]
pub enum StructEntry<'a> {
    Var(Variable<'a>),
    Function(Function<'a>),
}

#[derive(Debug)]
pub struct Struct<'a> {
    pub name: &'a str,
    pub inharit: Option<&'a str>,
    pub entries: Vec<StructEntry<'a>>,
}

#[derive(Debug)]
pub struct ApiDef<'a> {
    pub text: String,
    pub entries: Vec<Struct<'a>>,
}

impl<'a> ApiDef<'a> {
    pub fn parse_file<P: AsRef<Path>>(&'a mut self, path: P) {
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut self.text).unwrap();

        let mut parser = Rdp::new(StringInput::new(&self.text));

        parser.chunk();

        if !parser.end() {
            let expected = parser.expected();
            panic!("Failed to parse {:?} - {}", parser.expected(), &self.text[expected.1..]);
        }

        for token in parser.queue() {
            println!("{:?}", token);

            let s = Struct {
                name: &self.text[0..3],
                inharit: None,
                entries: Vec::new(),
            };

            self.entries.push(s);

            /*
            match token.rule {
            }
            */
        }

        // build up tokens here
    }
}


/*
fn main() {
    let mut parser = Rdp::new(StringInput::new(
        "
        // This is a comment!
        Rect {
           f32 x,
           f32 y,
           f32 width,
           f32 height,
       }

       // This struct has some functions and callbacks
       Foo {
            test2(i32 test, u32 foo),
            // Another comment in the \"struct\"
            [callback] test1(Rect test) -> void,
            i32 foo,
        }

        // Empty struct
        Table {

        }"));

    assert!(parser.block());
    assert!(parser.end());

    for token in parser.queue() {
        println!("{:?}", token);
    }
}
*/