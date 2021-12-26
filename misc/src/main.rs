mod topics;

use topics::print;
use topics::vars;
use topics::types;
use topics::strings;
use topics::tuples;
use topics::arrays;

fn main() {
    print::run();

    vars::run();

    types::run();
    
    strings::run();

    tuples::run();

    arrays::run();
}
