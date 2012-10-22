// Public Domain (-) 2012 The Rusty Authors.
// See the Rusty UNLICENSE file for details.

//= The optparse module provides support for handling command line options.
//
// Just specify the options that you want your program to take and `optparse`
// will automatically generate usage messages, validate arguments, type cast,
// provide auto-completion support and even handle sub-commands.
//
// To start, initialise a handler with `usage` and `version` info for your
// command line program, e.g.
//
//     let opts = optparse::new(
//                  "Usage: jsonprint [options] <files>", "jsonprint 0.1"
//                  );
//
// Then add the various options that you want it to handle by specifying the
// `flags` and a brief help `info` for the specific option type, e.g.
//
//     let indent = opts.int(["-i", "--indent"], "number of spaces to use for indentation");
//     let output = opts.str(["-o", "--output"], "the path to write the output to");
//     let pretty = opts.bool(["-p", "--pretty"], "pretty-print the generated output");
//
// And invoke the `optparse` machinery by calling the `parse()` method. This
// will return any left-over parameters as a string slice, e.g.
//
//     let files = opts.parse();
//
// You can then dereference the option variables to get their parsed values, e.g.
//
//     io::println(fmt!("Writing to the file: %s", *output));
//
//     if *pretty {
//         // pretty-print the output ...
//     }
//
// The builtin option types correspond to Rust's core data types and can be one
// of `bool`, `int`, 'i64', `str`, 'uint' or 'u64'. They default to the "zero"
// value for their type, i.e. `false`, `0` or `""`.
//
// You can override the default by specifying a third parameter when you
// initialise the options, e.g.
//
//     let indent = opts.int(
//         ["-i", "--indent"], "number of spaces to use for indentation", 4
//         );
//
// You can also specify that an option is required by calling `required()`
// before initialising the option, e.g.
//
//     let output = opts.required().str(["-o", "--output", "..."]);
//
// In order to be user-friendly, `-h`/`--help` and `-v`/`--version` options are
// automatically added when you call `parse()`. These use the `usage`, `version`
// and option `info` parameters you specified to auto-generate helpful output,
// e.g.
//
// ```
// $ jsonprint --help
// Usage: jsonprint [options] <files>
//
//   -h, --help        show this help message and exit
//   -v, --version     show program's version number and exit
// ```
//
// This is often the desired behaviour and follows established best practice for
// command line tools. Sometimes though, especially when serving non-English
// locales, you might want to disable this behaviour:
//
//     opts.add_help = false;
//     opts.add_version = false;
//
// If you want, you can still add custom options and handle it yourself with the
// utility `print_help()` and `print_version()` methods, e.g.
//
//     let help = opts.bool(["-h", "--hilfe"], "diese hilfe anzeigen und beenden");
//
//     opts.add_help = false;
//     opts.parse();
//
//     if *help {
//         opts.print_help();
//         os.set_exit_status(0);
//     }
//
// You can override the default parsing of the command line `os.args()` by
// passing an explicit string slice to `parse()`, e.g.
//
//     let files = opts.parse(["jsonprint", "items.json"]);
//
// Following convention, all arguments following a standalone `--` parameter are
// returned without being parsed as options, e.g in the following, `--pretty` is
// not treated as an option flag:
//
//     let files = opts.parse(["jsonprint", "items.json", "--", "--pretty"]);
//

// You can define custom option types by implementing the `Value` trait and
// specifying it in the explicit `option` initialiser, e.g. to aggregate
// multiple `--server` values into a vector, you might do something like:
//
//     type Servers = ~[~str];
//
//     impl Servers: optparse::Value {
//         fn set(arg: ~str) Option {
//             if arg.len() == 0 {
//                 return Some(~"server value cannot be empty");
//             }
//             self.push(arg);
//             return None;
//         }
//     }
//
//     let servers: Servers = ~[];
//
//     opts.option(["-s", "--server"], "address of upstream server", servers)

// auto-complete
// config file

use core::option::{None, Some};

type ErrPrinter = fn(&str, &str);

fn default_arg_required(prog: &str, arg: &str) {
    io::println(fmt!("%s: error: %s option requires an argument", prog, arg))
}

fn default_no_such_option(prog: &str, arg: &str) {
    io::println(fmt!("%s: error: no such option: %s", prog, arg))
}

fn default_required(prog: &str, arg: &str) {
    io::println(fmt!("%s: error: required: %s", prog, arg))
}

// The Value trait.
pub trait Value {
    fn set(&str) -> core::option::Option<~str>;
    fn string() -> ~str;
}

pub struct OptionParser {
    mut add_help: bool,
    mut add_version: bool,
    mut err_arg_required: ErrPrinter,
    mut err_no_such_option: ErrPrinter,
    mut err_required: ErrPrinter,
    mut set_required: bool,
    mut usage: ~str,
    mut version: ~str,
}

impl OptionParser {

    fn print_config_file(name: &str) {
        io::println(name)
    }

    fn int(flags: ~[~str], value: ~str, usage: ~str) {

    }

    // fn required() -> ~OptionParser {
    fn required() -> ~OptionParser/&self {
        self.set_required = true;
        return self
    }

}

struct Option {
    required: bool,
    value: Value
}

fn newopt() {

}

pub fn new(usage: ~str, version: ~str) -> ~OptionParser {
    ~OptionParser{
        add_help: true,
        add_version: if version == ~"" {
            false
        } else {
            true
        },
        err_arg_required: default_arg_required,
        err_no_such_option: default_no_such_option,
        err_required: default_required,
        set_required: false,
        usage: copy usage,
        version: copy version
    }
}