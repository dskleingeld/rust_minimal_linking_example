#[link(name = "libtest", kind = "static")]
extern "C" {
	fn testprint();
}

fn main() {
	unsafe { testprint() };
}
