//! SigmaOS-arch-shader-auditability-kernel — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_arch_shader_auditability_kernel_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-arch-shader-auditability-kernel: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-arch-shader-auditability-kernel: ok");
}
