fn main() {
  println!("#!/bin/bash");
  for q in 1..254 {
    for w in 1..254 {
      for e in 1..254 {
        for r in 1..254 {
          println!("ping -c 1 -i 0.2 {q}.{w}.{e}.{r}");
          println!("if [ $? -eq 0 ]");
          println!("then");
          println!("echo -e {q}.{w}.{e}.{r}\n | tee -a ips");
          println!("fi");
        }
      }
    }
  }
}
