# Overview
This project began as an example to work with Rust.  Below are some of the learnings resources I found helpful.

## General Rust resources
* https://doc.rust-lang.org/book/
* https://doc.rust-lang.org/style-guide/
* https://github.com/rust-lang/rustlings
* https://doc.rust-lang.org/rust-by-example/
* https://rust-lang-nursery.github.io/rust-cookbook/
* https://cheats.rs/
* https://dhghomon.github.io/easy_rust/

# CLI Specifics
* https://rust-cli-recommendations.sunshowers.io/index.html
* https://rust-cli.github.io/book/in-depth/config-files.html
* Colored terminal output: https://docs.rs/colored/latest/colored/

# Borrowing and References
* https://www.fpcomplete.com/blog/rust-asref-asderef/

# Rust Kubernetes 
* Kubeconf: https://docs.rs/kube-conf/latest/kube_conf/
  * https://github.com/esphen/kube-conf/blob/master/src/lib.rs
* Kube-rs: https://docs.rs/crate/kube/0.85.0
  * Kubeconfig: https://docs.rs/kube/latest/kube/config/struct.Kubeconfig.html
    * https://github.com/kube-rs/kube/blob/main/kube-client/src/config/mod.rs

# General Tips
* Clean up chains of Option or Result
  * https://doc.rust-lang.org/std/option/enum.Option.html#method.and
  * Convert from Result into Option:
    * https://stackoverflow.com/questions/28572101/what-is-a-clean-way-to-convert-a-result-into-an-option
  * https://doc.rust-lang.org/rust-by-example/flow_control/let_else.html
  * 