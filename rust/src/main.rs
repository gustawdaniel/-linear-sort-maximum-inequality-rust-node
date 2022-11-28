use linear_search_max_inequality::max_inequality;
use std::io;

fn main() -> io::Result<()>{
    max_inequality(&mut io::stdin(), &mut io::stdout())
}
