use std::{ffi::OsString,  path::{PathBuf}};

use crate::{types::{ShType, ShTypeDiscrim}, types_discriminant::TypeDiscrim};
/// the part of the command you can actually run
/// maybe not useful cause of std::Command, iunno
enum Executable{ // add env later when stuff can access variables
    // too much work for now
    Binary(PathBuf),
    Builtin(fn(Layout) /* -> pipe or something */), // @work_pipe;
    // i have no clue how to do pipes. later me problem lol work@
    Function(String) // user defined function
    // replace with token tree or whatever later maybe idk
}

// @work; allow for optional arguments that have bounded items
pub struct RigidArgs (Vec<ShType>); // @work make this more complicated
// @work_optional-rigid; make it so arguents can be optional
// probably gonna make it a single argument, then use Layout to handle multiple
// see; find, dd, bc, ls
// as examples of different kinds of commands that may be used and weird
// soft args may also need to be changed
// iunno
// find is the main outlier i can think of
// there's a lot of commands i don't know of


pub struct SoftArg { // often called Flags or switches
    // oft notated as --flag and -switch, respectively
    // values can be given basically any way you want
    // usually --flag=val, -switch=val, or the same thing with spaces
    // some commands, like dd, use other syntax, eg;
    // 'dd if=foo of=bar' is the same as
    // 'dd of=bar if=foo'; these are soft arguments, the order doesn't matter
    // which is why it's not called a flag nor a switch, those are standards
    // SoftArg is just anything that doesn't need order
    flag: String,
    short: Option<String>,
    // may be empty, but not very expensive to handle when empty
    // so no option, it just handles an empty vec
    // (there may be any number of arguments, 0 -> some huge number)
    arg: RigidArgs,
    // if None, send '--flag' and 'value' as two arguments
    // if Some, send '--flag'<joined_string>'value' as one argument
    joined: Option<String>
}

/// describes the inputs of a command
pub struct Layout { // @work_rename-layout; arguments? and then call template layout?
    // arguments which have positions
    args: RigidArgs,
    // flags which do not
    // eventually i'll probably add a way to do pre/post flag stuff @work
    // (like with find, flags (switches) (-name, -exec, etc) 
    // have to be after the filepath, but the order of them doesn't matter)
    flags: Vec<SoftArg>
}

// @split; maybe split this?
// into a layout file, a command file, and a template file?
// unsure whether good idea or not

/// it's like the stuff above except it doesn't have values
pub struct TemplateRigid (Vec<ShTypeDiscrim>);
pub struct TemplateSoft {
    flag: String,
    short: Option<String>,
    arg: TemplateRigid, // only difference
    joined: Option<String>
}
pub struct Template {
    args: TemplateRigid,
    flags: Vec<TemplateSoft>
}
impl Layout {
    /// does the layout conform to the bounds of the template
    fn follows_template(&self, compare: Template) -> bool {
        // @work; make this have acual errors.
        // or maybe have a spereate function for that, iunno
        // work@ 
        // first check the rigid arguments
        // (faster and more likely to fail)
        let layout_args = &self.args.0;
            let largs_len = layout_args.len();
        let template_args = compare.args.0;
            let targs_len = template_args.len();
        // if the amount of rigid arguments isn't the same,
        // then it's can't conform since those are non optional
        // (these will be optional later)
        // (it will then need to be parsed as next_matching)
        // @work_optional-rigid
        if layout_args.len() != template_args.len() {
            return false // not the same length, so doesn't follow template
            // @feature try and coerce wrong types into matching when possible
        }
        for i in 0..largs_len { 
            let layout_type = &Into::<ShTypeDiscrim>::into(&layout_args[i]);
            // template type is already a discriminant
            let template_type = &template_args[i];
            if layout_type != template_type {
                return false; // wrong type, doesn't match the template
            }
        };
        //@bookmark; check soft args, order agnostic
        todo!()
    }
}
struct Command {
    name: OsString,
    exec: Executable,
    layout: Option<Layout>
}

impl Command {
    // @work make it so i can run stuff!!!
    // or something iunno work@
    // @split definitely split this into a commands file and a layout.rs file
    // also do the thing previously mentioned somewhere about renaming template to layout
    // and layout to something else split@
}