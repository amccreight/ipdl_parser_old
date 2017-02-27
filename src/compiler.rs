/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate regex;

use ast::{Direction, FileType, Protocol, StructField, TranslationUnit, TypeSpec, UsingStmt, Location};
use std::collections::HashMap;
use std::iter;
use std::path::PathBuf;
use parser;
use self::regex::Regex;
use type_check;

const MESSAGE_START_HEADER: &'static str = "IPCMessageStart.h";
const MESSAGE_TYPE_DEFNS: &'static str = "IPCMessageTypeName.cpp";

pub fn compile(include_dirs: &Vec<PathBuf>, file_names: Vec<PathBuf>) -> Option<HashMap<String, String>> {
    let maybe_tus = parser::parse(&include_dirs, file_names);

    if maybe_tus.is_none() {
        println!("Specification could not be parsed.");
        return None;
    }

    let tus = maybe_tus.unwrap();
    if let Err(err) = type_check::check(&include_dirs, &tus) {
        println!("Error(s) during type checking.\n{}", err);
        return None;
    }

    let mut files_to_write = HashMap::new();
    let mut all_message_strs: Vec<String> = vec![];
    for (ipdl_path, tu) in tus {
        match compile_translation_unit(&ipdl_path, tu) {
            Some((mapping, messages)) => {
                files_to_write.extend(mapping);
                all_message_strs.extend(messages);
            },
            None => {
                println!("No TranslationUnit generated for {}", ipdl_path.display());
                return None;
            },
        }
    }

    // IPCMessageStart.h and IPCMessageTypeName.cpp are
    // special files that we'll create after iterating
    // all TranslationUnits. We'd better not have compiled
    // any TranslationUnits that will result in files with
    // those names.
    assert!(!files_to_write.contains_key(MESSAGE_START_HEADER),
            "Should not have the MESSAGE_START_HEADER yet.");
    assert!(!files_to_write.contains_key(MESSAGE_TYPE_DEFNS),
            "Should not have the MESSAGE_TYPE_DEFNS yet.");

    files_to_write.insert(MESSAGE_START_HEADER.to_owned(),
        compile_message_start_header(&all_message_strs).to_owned());

    files_to_write.insert(MESSAGE_TYPE_DEFNS.to_owned(),
        compile_message_type_defns(&all_message_strs).to_owned());

    for (filename, contents) in &files_to_write {
        println!("Write: {}", filename.to_owned());
        println!("With contents: ");
        println!("{}\n\n", contents.to_owned());
    }
    return Some(files_to_write);
}

fn compile_translation_unit(ipdl_path: &PathBuf, tu: TranslationUnit) -> Option<(HashMap<String, String>, Vec<String>)> {
    let mut files_to_write: HashMap<String, String> = HashMap::new();

    // Let's compute what the generated file names will be. For files called
    // PSomething.ipdl, we'll produce the following files:
    //
    // PSomething.h
    // PSomething.cpp
    // PSomethingParent.h
    // PSomethingParent.cpp
    // PSomethingChild.h
    // PSomethingChild.cpp
    //
    // "PSomething", in this case, is the "stem".
    /*
    let stem = ipdl_path.file_stem().expect("Couldn't get file stem.");
    let base = stem.to_str().expect("Could not convert file stem to str").clone();
    let parent = format!("{}Parent", base);
    let child = format!("{}Child", base);

    let mut path = ipdl_path.clone();
    path.set_file_name(parent);

    result.insert(path.clone().to_owned(), "WOO".to_owned());
    //result.insert(ipdl_path.set_extension(child).clone(), "WOO".to_owned());
    //result.insert(ipdl_path.set_extension(base).clone(), "WOO".to_owned());
    */
    let mut message_strs: Vec<String> = vec![];
    // See if this TranslationUnit has a protocol. .ipdl files have
    // protocols defined in them, but .ipdlh files do not.
    if tu.protocol.is_some() {
      let protocol_tuple = tu.protocol.expect("Should have had a protocol");
      let namespaces = protocol_tuple.0.namespaces.join("");
      let messages = protocol_tuple.1.messages;
      for message in messages {
        message_strs.push(format!("{}{}", namespaces, message.name.id).to_owned());
      }
    }

    return Some((files_to_write, message_strs));
}

fn template(template_str: &str, replacements: HashMap<&str, &str>) -> String {
  for (key, insertion) in replacements {
    let key_match = format!("{{{{ {} }}}}", "INDENTED_MESSAGES");
    return template_str.replace(&key_match, insertion);
  }

  return template_str.to_string();
}

fn indent(source: &str, depth: usize) -> String {
  lazy_static! {
    static ref LINE_START_RE: Regex = Regex::new(r"^(?P<s>[^\n#])").unwrap();
  }

  let indents: String = iter::repeat(" ").take(depth).collect();
  let replacement = format!("{}$s", indents);
  return LINE_START_RE.replace_all(source, replacement.as_str()).to_owned();
}

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

fn compile_message_start_header(all_messages: &Vec<String>) -> String {
  let messages = indent(&all_messages.join(",\n"), 2);
  let s = template(r##"\
// CODE GENERATED by ipdl_parser. Do not edit.

#ifndef IPCMessageStart_h
#define IPCMessageStart_h

enum IPCMessageStart {
{{ INDENTED_MESSAGES }},
  LastMsgIndex
};

static_assert(LastMsgIndex <= 65536, "need to update IPC_MESSAGE_MACRO");

#endif // ifndef IPCMessageStart_h
"##, map!{ "INDENTED_MESSAGES" => messages.as_str() });
  return s.to_owned();
}

fn compile_message_type_defns(all_messages: &Vec<String>) -> String {
    return "Hey, these are the message type definitions!".to_owned();
}
