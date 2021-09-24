// run-pass
// edition:2018

async fn parameter_no_drop_no_use(_a: u32) {
    std::future::ready(()).await;
}

async fn parameter_explicit_drop(a: u32) {
    std::future::ready(()).await;
    drop(a);
}

struct HasDrop(u32);

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("drop");
    }
}

async fn parameter_implicit_drop(_a: HasDrop) {
    std::future::ready(()).await;
}

async fn parameter_rebind(a: HasDrop) {
    let _x = a;
    std::future::ready(()).await;
}

fn main() {
    assert_eq!(8, std::mem::size_of_val(&parameter_no_drop_no_use(0)));
    assert_eq!(8, std::mem::size_of_val(&parameter_explicit_drop(0)));
    assert_eq!(8, std::mem::size_of_val(&parameter_implicit_drop(HasDrop(0))));
    assert_eq!(16, std::mem::size_of_val(&parameter_rebind(HasDrop(0))));
}
