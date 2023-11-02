# HelloWorld-Rust
A basic "Hello World" program, written in [Rust](https://www.rust-lang.org/).

## Getting up and running
To start an Eclipse Che workspace, you can simply pass this repository's URL to the Che server using the [factory url API](https://www.eclipse.org/che/docs/che-7/configuring-a-workspace-using-a-devfile/#creating-a-workspace-from-the-default-branch-of-a-git-repository_configuring-a-workspace-using-a-devfile), e.g. 

che.openshift.io/f?url=https://github.com/che-samples/helloworld-rust

## Building
If using Che to develop the project, you can use the built-in commands `build` and `run`. Alternatively, you can use [`cargo`](https://doc.rust-lang.org/cargo/):

```bash
# build
cargo build
# run
cargo run
```