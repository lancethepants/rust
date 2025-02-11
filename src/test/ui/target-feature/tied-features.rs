// build-fail
// compile-flags: --crate-type=rlib --target=aarch64-unknown-linux-gnu
// needs-llvm-components: aarch64
#![feature(aarch64_target_feature, target_feature_11)]
#![feature(no_core, lang_items)]
#![no_core]

#[lang="sized"]
trait Sized {}

// FIXME: this should not need to be public.
pub fn main() {
    #[target_feature(enable = "pacg")]
    //~^ ERROR must all be either enabled or disabled together
    unsafe fn inner() {}

    unsafe {
        foo();
        bar();
        baz();
        inner();
    }
}

#[target_feature(enable = "paca")]
//~^ ERROR must all be either enabled or disabled together
unsafe fn foo() {}


#[target_feature(enable = "paca,pacg")]
unsafe fn bar() {}

#[target_feature(enable = "paca")]
#[target_feature(enable = "pacg")]
unsafe fn baz() {}
