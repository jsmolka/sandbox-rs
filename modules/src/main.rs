mod inline;
mod syntax_new;
mod syntax_old;

fn main() {
    syntax_old::one::func();
    syntax_old::two::func();
    syntax_old::func();
    syntax_new::one::func();
    syntax_new::two::func();
    syntax_new::func();
    inline::one::func();
    inline::two::func();
    inline::func();
}
