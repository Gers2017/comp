use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const COMP_VERSION: &str = "0.18.0";

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

// -- command list -------------------------------------------------------------

const CMDS: &str = "drop dup swap cls clr roll sa .a a sb .b b sc .c c + +_ - x \
x_ / chs abs round int inv sqrt throot proot ^ exp % mod ! gcd pi e dtor rtod \
sin asin cos acos tan atan log log10 ln";


fn main() {
  let mut args: Vec<String> = env::args().collect();

  // create computation processor with stack, memory slots and an operations list
  let mut processor = Processor{
                     stack: Vec::new(),
                     mem_a: 0.0,
                     mem_b: 0.0,
                     mem_c: 0.0,
                     ops: Vec::new(),
                   };


  if args.len() <= 1 {
    args.push("help".to_string());
  }

  if args[1] == "--help" || args[1] == "help" {
    // display command usage information
    println!("usage: comp [version] [help]");
    println!("       comp <list>");
    println!("       comp -f <file>");
    println!();
    println!("where <list> represents a sequence of reverse Polish notion (RPN) \
    postfix operations or <file> is a file containing a similar sequence of \
    operations. Each operation must be either a command (symbol) or value. As \
    examples, 'comp 3 4 +' adds the values 3 and 4 and '3 dup x 4 dup x +' \
    computes the sum of the squares of 3 and 4. The available commands are \
    listed below.");
    println!();
    println!("commands:");
    println!("{}", CMDS);

    return;
  } else if args[1] == "--version" || args[1] == "version" {
    // display version information
    println!("comp {}", COMP_VERSION);
    return;
  } else if args[1] == "mona" {
    println!("{}", MONA);
    return;
  } else if args[1] == "-f" || args[1] == "--file" {
    // read operations list input from file
    print!("reading command input from '{}' file .. ", args[2].to_string()); // debug

    let filename = args[2].to_string();
    let path = Path::new(&filename);
    let display = path.display();

    let mut file = match File::open(&path) {
                     Err(why) => panic!("couldn't open {}: {}", display, why),
                     Ok(file) => file,
                   };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
      Err(why) => panic!("couldn't read {}: {}", display, why),
      Ok(_) => println!("success"),
    };

    let temp_ops: Vec<&str> = contents.split_whitespace().collect();

    // create operations list vector
    for op in temp_ops {
      processor.ops.push(op.to_string());
    }
  } else {
    // read operations list input from command line arguments
    processor.ops = (&args[1..]).to_vec(); //remove
  }
  //println!("{:?}", processor.ops); // debug message

  // process operations list
  processor.process_ops();

  // display resulting computation stack
  for element in processor.stack {
    println!("{}", element);
  }
}

struct Processor {
  stack: Vec<f64>,
  mem_a: f64,
  mem_b: f64,
  mem_c: f64,
  ops: Vec<String>,
}

impl Processor {
  fn process_ops(&mut self) {
    //println!("process_ops() - self.ops = {:?}", self.ops); // debug message

    while self.ops.len() >= 1 {
      //let operation: String = self.ops.pop().unwrap().clone();
      let operation: String = self.ops.remove(0);
      self.processnode(operation.as_str());
    }
  }

  fn processnode(&mut self, op: &str) {
    //println!("processnode() - op = {}, self.stack = {:?}", op, self.stack); // debug message

    match op {
      // stack manipulation
      "drop"   => self.c_drop(),      // drop
      "dup"    => self.c_dup(),       // duplicate
      "swap"   => self.c_swap(),      // swap x and y
      "cls"    => self.c_cls(),       // clear stack
      "clr"    => self.c_cls(),       // clear stack
      "roll"   => self.c_roll(),      // roll stack
      // memory usage
      "sa"     => self.c_store_a(),   // store (pop value off stack and store)
      ".a"     => self.c_store_a(),   // store (pop value off stack and store)
      "a"      => self.c_push_a(),    // retrieve (push stored value onto the stack)
      "sb"     => self.c_store_b(),   // store
      ".b"     => self.c_store_b(),   // store
      "b"      => self.c_push_b(),    // retrieve
      "sc"     => self.c_store_c(),   // store
      ".c"     => self.c_store_c(),   // store
      "c"      => self.c_push_c(),    // retrieve
      // math operations
      "+"      => self.c_add(),       // add
      "+_"     => self.c_add_all(),   // add all
      "-"      => self.c_sub(),       // subtract
      "x"      => self.c_mult(),      // multiply
      "x_"     => self.c_mult_all(),  // multiply all
      "/"      => self.c_div(),       // divide
      "chs"    => self.c_chs(),       // change sign
      "abs"    => self.c_abs(),       // absolute value
      "round"  => self.c_round(),     // round
      "int"    => self.c_round(),
      "inv"    => self.c_inv(),       // invert (1/x)
      "sqrt"   => self.c_sqrt(),      // square root
      "throot" => self.c_throot(),    // nth root
      "proot"  => self.c_proot(),     // find principal roots
      "^"      => self.c_exp(),       // exponenation
      "exp"    => self.c_exp(),
      "%"      => self.c_mod(),       // modulus
      "mod"    => self.c_mod(),
      "!"      => self.c_fact(),      // factorial
      "gcd"    => self.c_gcd(),       // greatest common divisor
      "pi"     => self.c_pi(),        // pi
      "e"      => self.c_euler(),     // Euler's constant
      "dtor"   => self.c_dtor(),      // degrees to radians
      "rtod"   => self.c_rtod(),      // radians to degrees
      "sin"    => self.c_sin(),       // sine
      "asin"   => self.c_asin(),      // arcsine
      "cos"    => self.c_cos(),       // cosine
      "acos"   => self.c_acos(),      // arccosine
      "tan"    => self.c_tan(),       // tangent
      "atan"   => self.c_atan(),      // arctangent
      "log"    => self.c_log10(),     // log (base 10)
      "log10"  => self.c_log10(),
      "ln"     => self.c_ln(),        // natural log
      _ => self.stack.push(op.parse::<f64>().unwrap()), // push value onto stack
    }
  }

  // -- command functions --------------------------------------------------------
  
  // ---- stack manipulation -----------------------------------------------------
  
  fn c_drop(&mut self) {
    self.stack.pop().unwrap();
  }
  
  fn c_dup(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack.push(self.stack[end]);
  }
  
  fn c_swap(&mut self) {
    let end: usize = self.stack.len() - 1;
    let o: f64 = self.stack[end];
    self.stack[end] = self.stack[end-1];
    self.stack[end-1] = o;
  }
  
  fn c_cls(&mut self) {
    self.stack.clear();
  }
  
  fn c_roll(&mut self) {
    let o: f64 = self.stack.pop().unwrap();
    self.stack.splice(0..0, [o]);
  }
  
  
  // ---- memory usage -----------------------------------------------------------
  
  fn c_store_a(&mut self) {
    self.mem_a = self.stack.pop().unwrap();
  }
  
  fn c_push_a(&mut self) {
    self.stack.push(self.mem_a);
  }
  
  fn c_store_b(&mut self) {
    self.mem_b = self.stack.pop().unwrap();
  }
  
  fn c_push_b(&mut self) {
    self.stack.push(self.mem_b);
  }
  
  fn c_store_c(&mut self) {
    self.mem_c = self.stack.pop().unwrap();
  }
  
  fn c_push_c(&mut self) {
    self.stack.push(self.mem_c);
  }
  
  
  // -- math operations ----------------------------------------------------------
  
  fn c_add(&mut self) {
    //println!("c_add() - self.stack = {:?}", self.stack); // debug message
    //println!("c_add() - self.ops = {:?}", self.ops); // debug message
    let end: usize = self.stack.len() - 1;
    self.stack[end-1] += self.stack.pop().unwrap();
  }
  
  fn c_add_all(&mut self) {
    while self.stack.len() > 1 {
      let end: usize = self.stack.len() - 1;
      self.stack[end-1] += self.stack.pop().unwrap();
    }
  }
  
  fn c_sub(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end-1] -= self.stack.pop().unwrap();
  }
  
  fn c_mult(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end-1] *= self.stack.pop().unwrap();
  }
  
  fn c_mult_all(&mut self) {
    while self.stack.len() > 1 {
      let end: usize = self.stack.len() - 1;
      self.stack[end-1] *= self.stack.pop().unwrap();
    }
  }
  
  fn c_div(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end-1] /= self.stack.pop().unwrap();
  }
  
  fn c_chs(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end] *= -1.0;
  }
  
  fn c_abs(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end] = f64::abs(self.stack[end]);
  }
  
  fn c_round(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].round();
  }
  
  fn c_inv(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end] = 1.0 / self.stack[end];
  }
  
  fn c_sqrt(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end] = f64::sqrt(self.stack[end]);
  }
  
  fn c_throot(&mut self) {
    let end: usize = self.stack.len() - 1;
    let o: f64 = self.stack.pop().unwrap();
    self.stack[end-1] = self.stack[end-1].powf(1.0/o);
  }
  
  fn c_proot(&mut self) {
    let c: f64 = self.stack.pop().unwrap();
    let b: f64 = self.stack.pop().unwrap();
    let a: f64 = self.stack.pop().unwrap();
  
    if (b*b - 4.0*a*c) < 0.0 {
      self.stack.push(-1.0*b/(2.0*a)); // root1 real
      self.stack.push(f64::sqrt(4.0*a*c-b*b)/(2.0*a)); // root1 imag
      self.stack.push(-1.0*b/(2.0*a)); // root2 real
      self.stack.push(-1.0*f64::sqrt(4.0*a*c-b*b)/(2.0*a)); // root2 imag
    } else {
      self.stack.push(-1.0*b+f64::sqrt(b*b-4.0*a*c)/(2.0*a)); // root1 real
      self.stack.push(0.0); // root1 imag
      self.stack.push(-1.0*b-f64::sqrt(b*b-4.0*a*c)/(2.0*a)); // root2 real
      self.stack.push(0.0); // root2 imag
    }
  }
  
  fn c_exp(&mut self) {
    let end: usize = self.stack.len() - 1;
    let o: f64 = self.stack.pop().unwrap();
    self.stack[end-1] = self.stack[end-1].powf(o);
  }
  
  fn c_mod(&mut self) {
    let end: usize = self.stack.len() - 1;
    let o: f64 = self.stack.pop().unwrap();
    self.stack[end-1] = self.stack[end-1] % o;
  }
  
  fn c_fact(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end] = factorial(self.stack[end] as u64) as f64;
  }
  
  fn c_gcd(&mut self) {
    let a: u64 = self.stack.pop().unwrap() as u64;
    let b: u64 = self.stack.pop().unwrap() as u64;
    let g: f64 = gcd(a,b) as f64;
    self.stack.push(g);
  }
  
  fn c_pi(&mut self) {
    self.stack.push(std::f64::consts::PI);
  }
  
  fn c_euler(&mut self) {
    self.stack.push(std::f64::consts::E);
  }
  
  fn c_dtor(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].to_radians();
  }
  
  fn c_rtod(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].to_degrees();
  }
  
  fn c_sin(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].sin();
  }
  
  fn c_asin(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].asin();
  }
  
  fn c_cos(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].cos();
  }
  
  fn c_acos(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].acos();
  }
  
  fn c_tan(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].tan();
  }
  
  fn c_atan(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].atan();
  }
  
  fn c_log10(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].log10();
  }
  
  fn c_ln(&mut self) {
    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].ln();
  }


}


// -- support functions --------------------------------------------------------

fn factorial(n: u64) -> u64 {
  if n < 2 {
    return 1;
  } else {
    return n * factorial(n-1);
  }
}

fn gcd(a: u64, b: u64) -> u64 {
  if b != 0 {
    return gcd(b, a % b)
  } else {
    return a
  }
}


// -- mona ---------------------------------------------------------------------

const MONA: &str = "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!>''''''<!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
       !!!!!!!!!!!!!!!!!!!!!!!!!!!!'''''`             ``'!!!!!!!!!!!!!!!!!!!!!!!!\n\
       !!!!!!!!!!!!!!!!!!!!!!!!''`          .....         `'!!!!!!!!!!!!!!!!!!!!!\n\
       !!!!!!!!!!!!!!!!!!!!!'`      .      :::::'            `'!!!!!!!!!!!!!!!!!!\n\
       !!!!!!!!!!!!!!!!!!!'     .   '     .::::'                `!!!!!!!!!!!!!!!!\n\
       !!!!!!!!!!!!!!!!!'      :          `````                   `!!!!!!!!!!!!!!\n\
       !!!!!!!!!!!!!!!!        .,cchcccccc,,.                       `!!!!!!!!!!!!\n\
       !!!!!!!!!!!!!!!     .-\"?$$$$$$$$$$$$$$c,                      `!!!!!!!!!!!\n\
       !!!!!!!!!!!!!!    ,ccc$$$$$$$$$$$$$$$$$$$,                     `!!!!!!!!!!\n\
       !!!!!!!!!!!!!    z$$$$$$$$$$$$$$$$$$$$$$$$;.                    `!!!!!!!!!\n\
       !!!!!!!!!!!!    <$$$$$$$$$$$$$$$$$$$$$$$$$$:.                    `!!!!!!!!\n\
       !!!!!!!!!!!     $$$$$$$$$$$$$$$$$$$$$$$$$$$h;:.                   !!!!!!!!\n\
       !!!!!!!!!!'     $$$$$$$$$$$$$$$$$$$$$$$$$$$$$h;.                   !!!!!!!\n\
       !!!!!!!!!'     <$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$                   !!!!!!!\n\
       !!!!!!!!'      `$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$F                   `!!!!!!\n\
       !!!!!!!!        c$$$$???$$$$$$$P\"\"  \"\"\"??????\"                      !!!!!!\n\
       !!!!!!!         `\"\" .,.. \"$$$$F    .,zcr                            !!!!!!\n\
       !!!!!!!         .  dL    .?$$$   .,cc,      .,z$h.                  !!!!!!\n\
       !!!!!!!!        <. $$c= <$d$$$   <$$$$=-=+\"$$$$$$$                  !!!!!!\n\
       !!!!!!!         d$$$hcccd$$$$$   d$$$hcccd$$$$$$$F                  `!!!!!\n\
       !!!!!!         ,$$$$$$$$$$$$$$h d$$$$$$$$$$$$$$$$                   `!!!!!\n\
       !!!!!          `$$$$$$$$$$$$$$$<$$$$$$$$$$$$$$$$'                    !!!!!\n\
       !!!!!          `$$$$$$$$$$$$$$$$\"$$$$$$$$$$$$$P>                     !!!!!\n\
       !!!!!           ?$$$$$$$$$$$$??$c`$$$$$$$$$$$?>'                     `!!!!\n\
       !!!!!           `?$$$$$$I7?\"\"    ,$$$$$$$$$?>>'                       !!!!\n\
       !!!!!.           <<?$$$$$$c.    ,d$$?$$$$$F>>''                       `!!!\n\
       !!!!!!            <i?$P\"??$$r--\"?\"\"  ,$$$$h;>''                       `!!!\n\
       !!!!!!             $$$hccccccccc= cc$$$$$$$>>'                         !!!\n\
       !!!!!              `?$$$$$$F\"\"\"\"  `\"$$$$$>>>''                         `!!\n\
       !!!!!                \"?$$$$$cccccc$$$$??>>>>'                           !!\n\
       !!!!>                  \"$$$$$$$$$$$$$F>>>>''                            `!\n\
       !!!!!                    \"$$$$$$$$???>'''                                !\n\
       !!!!!>                     `\"\"\"\"\"                                        `\n\
       !!!!!!;                       .                                          `\n\
       !!!!!!!                       ?h.\n\
       !!!!!!!!                       $$c,\n\
       !!!!!!!!>                      ?$$$h.              .,c\n\
       !!!!!!!!!                       $$$$$$$$$hc,.,,cc$$$$$\n\
       !!!!!!!!!                  .,zcc$$$$$$$$$$$$$$$$$$$$$$\n\
       !!!!!!!!!               .z$$$$$$$$$$$$$$$$$$$$$$$$$$$$\n\
       !!!!!!!!!             ,d$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$          .\n\
       !!!!!!!!!           ,d$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$         !!\n\
       !!!!!!!!!         ,d$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$        ,!'\n\
       !!!!!!!!>        c$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$.\n\
       !!!!!!''       ,d$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$      allen mullen";


// -- unit and regression tests ------------------------------------------------

#[cfg(test)]

mod comp_tests {

  #[test]
  fn test_core() {
    let mut testcs = super::Processor{
                       stack: Vec::new(),
                       mem_a: 20.0,
                       mem_b: 6.18,
                       mem_c: -123.45,
                     };

    testcs.stack.push(1.0);
    testcs.stack.push(2.0);
    testcs.stack.push(3.0);
    testcs.stack.push(4.0);

    super::c_dtor(&mut testcs);
    super::c_cos(&mut testcs);
    super::c_acos(&mut testcs);
    super::c_sin(&mut testcs);
    super::c_asin(&mut testcs);
    super::c_tan(&mut testcs);
    super::c_atan(&mut testcs);
    super::c_rtod(&mut testcs);
    super::c_round(&mut testcs);
    super::c_roll(&mut testcs);
    super::c_roll(&mut testcs);
    super::c_roll(&mut testcs);
    super::c_roll(&mut testcs);
    super::c_dup(&mut testcs);
    super::c_drop(&mut testcs);
    super::c_swap(&mut testcs);
    super::c_swap(&mut testcs);
    super::c_add(&mut testcs);
    super::c_sub(&mut testcs);
    super::c_div(&mut testcs);

    assert!(testcs.stack.pop().unwrap() == -0.2);
  }

  #[test]
  fn test_support() {
    assert!(super::gcd(55, 10) == 5);
    assert!(super::factorial(10) == 3628800);
  }

  #[test]
  fn test_roots() {
    let mut testcs = super::Processor{
                       stack: Vec::new(),
                       mem_a: 0.1,
                       mem_b: 0.2,
                       mem_c: 0.3,
                     };

    testcs.stack.push(2.0);
    super::c_dup(&mut testcs);
    super::c_sqrt(&mut testcs);
    super::c_swap(&mut testcs);
    testcs.stack.push(32.0);
    super::c_exp(&mut testcs);
    testcs.stack.push(32.0 * 2.0);
    super::c_throot(&mut testcs);

    assert!(testcs.stack.pop().unwrap() == testcs.stack.pop().unwrap());

    testcs.stack.push(1.0);
    testcs.stack.push(-2.0);
    super::c_chs(&mut testcs);
    super::c_chs(&mut testcs);
    super::c_pi(&mut testcs);
    super::c_mult(&mut testcs);
    super::c_pi(&mut testcs);
    testcs.stack.push(2.0);
    super::c_exp(&mut testcs);
    testcs.stack.push(1.0);
    super::c_add(&mut testcs);
    super::c_proot(&mut testcs);
    super::c_add_all(&mut testcs);
    testcs.stack.push(2.0);
    super::c_div(&mut testcs);
    super::c_pi(&mut testcs);

    assert!(testcs.stack.pop().unwrap() == testcs.stack.pop().unwrap());
  }

  #[test]
  #[should_panic]
  fn test_cls() {
    let mut testcs = super::Processor{
                       stack: Vec::new(),
                       mem_a: 3.3,
                       mem_b: 4.4,
                       mem_c: 5.5,
                     };

    testcs.stack.push(1.0);
    testcs.stack.push(2.0);
    testcs.stack.push(3.0);
    testcs.stack.push(4.0);
    testcs.stack.push(1.0);
    testcs.stack.push(2.0);
    testcs.stack.push(3.0);
    testcs.stack.push(4.0);
    testcs.stack.push(1.0);
    testcs.stack.push(2.0);
    testcs.stack.push(3.0);
    testcs.stack.push(4.0);
    super::c_cls(&mut testcs);

    assert!(testcs.stack.pop().unwrap() == 0.0);
  }

  #[test]
  fn test_mem() {
    let mut testcs = super::Processor{
                       stack: Vec::new(),
                       mem_a: 8.88888,
                       mem_b: 8.88888,
                       mem_c: 8.88888,
                     };

    testcs.stack.push(1.0);
    testcs.stack.push(2.0);
    testcs.stack.push(3.0);
    testcs.stack.push(4.0);
    testcs.stack.push(1.0);
    testcs.stack.push(2.0);
    testcs.stack.push(3.0);
    testcs.stack.push(4.0);
    testcs.stack.push(1.0);
    testcs.stack.push(2.0);
    testcs.stack.push(3.0);
    testcs.stack.push(4.0);
    super::c_chs(&mut testcs);
    super::c_abs(&mut testcs);
    super::c_inv(&mut testcs);
    super::c_inv(&mut testcs);
    super::c_pi(&mut testcs);
    super::c_euler(&mut testcs);
    testcs.stack.push(0.0);
    super::c_store_b(&mut testcs); // 0
    super::c_store_a(&mut testcs); // e
    super::c_store_c(&mut testcs); // pi
    super::c_cls(&mut testcs);
    super::c_push_b(&mut testcs); // 0
    super::c_push_c(&mut testcs); // pi
    super::c_add(&mut testcs);
    super::c_push_a(&mut testcs); // e
    super::c_add(&mut testcs);

    assert!(testcs.stack.pop().unwrap() == std::f64::consts::PI + std::f64::consts::E);
  }

  #[test]
  fn test_cmp() {
      let mut testcs = super::Processor{
                         stack: Vec::new(),
                         mem_a: 0.0,
                         mem_b: 0.0,
                         mem_c: 0.0,
                       };

    testcs.stack.push(10.0);
    super::c_log10(&mut testcs);
    super::c_euler(&mut testcs);
    super::c_ln(&mut testcs);
    testcs.stack.push(105.0);
    testcs.stack.push(2.0);
    super::c_mod(&mut testcs);
    testcs.stack.push(3049.0);
    testcs.stack.push(1009.0);
    super::c_gcd(&mut testcs);
    super::c_mult_all(&mut testcs);

    assert!(testcs.stack.pop().unwrap() == 1.0);

    testcs.stack.push(20.0);
    super::c_fact(&mut testcs);

    assert!(testcs.stack.pop().unwrap() == 2432902008176640000.0);
  }
}
