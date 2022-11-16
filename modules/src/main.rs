use crate::{instance::Instance, shared::Shared};

mod instance;
mod shared;
mod syntax_new;
mod syntax_old;

fn main() {
    let instance = Instance {};
    instance.shared();

    syntax_old::one::func();
    syntax_old::two::func();
    syntax_old::func();
    syntax_new::one::func();
    syntax_new::two::func();
    syntax_new::func();
}
