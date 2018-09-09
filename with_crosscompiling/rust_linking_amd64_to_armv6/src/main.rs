#[link(name = "test", kind = "static")]
extern "C" {
	fn testprint();
}

fn main() {
	unsafe { testprint() };
}
