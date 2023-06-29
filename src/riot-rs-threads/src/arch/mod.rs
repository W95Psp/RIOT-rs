cfg_if::cfg_if! {
    if #[cfg(context = "cortex-m")] {
        mod cortex_m;
        pub use self::cortex_m::*;
    }
    else {
        pub(crate) fn setup_stack(_stack: &mut [u8], _func: usize, _arg: usize) -> usize {
            unimplemented!()
        }
        pub fn schedule() {
            unimplemented!();
        }
        pub(crate) fn start_threading(_next_sp: usize) {
            unimplemented!();
        }
    }
}
