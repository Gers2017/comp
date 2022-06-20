use std::env;

/*

    note: base data structure is a vector (linked
    list) used as a stack. atoms on the list are
    either be symbols (commands) or values. each
    calculation is a list of operations that are
    processed in order of occurrence. this is an
    implementation of a lisp interpreter for rev-
    erse polish notation s-expressions (sexp).

      operations list structure
        (object : command or value)
        "5"
        "sqrt"
        "1
        "-"
        "2"
        "/"

    a list evaluation engine takes the list of
    strings and executes the corresponding oper-
    ations then returns the resulting mutated
    stack.

*/

fn main() {
  const COMP_VERSION: &str = "0.15.0";

  let args: Vec<String> = env::args().collect();

  //println!("{:?}", args);

  let ops = &args[1..]; // operations list
  println!("{:?}", ops);
  //println!("{}", ops[0]);

  let mut cstack: Vec<f64> = Vec::new(); // computation stack

  ops.iter().map(|op| processnode(&cstack, &op)).collect::<Vec<_>>();
}

fn processnode(stack: &Vec<f64>, cmdval: &String) {
  println!("op = {}", cmdval);
}
