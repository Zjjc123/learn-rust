// rust does not see files and subfiles but modules and submodules
// create a tree of "pub mod file_name" that exposes modules to outer ones
mod types;
// use keyword uses a resource directly
use types::arrays;

fn main() {
    // print::run();

    // vars::run();

    // types::run();
    
    // strings::run();

    // tuples::run();

    arrays::run();
}
