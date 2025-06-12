fn main()
{
    // no_std feature
    println!("cargo::rustc-check-cfg=cfg(no_std)");
    if cfg!(feature = "no_std")
    {
        println!("cargo::rustc-cfg=no_std");
    }
}
