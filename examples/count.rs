#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), &'static str> {
    use std::{
        rc::Rc,
        sync::atomic::{AtomicUsize, Ordering},
    };

    pretty_env_logger::init();
    let counter = Rc::new(AtomicUsize::new(0));
    again::retry(move || {
        let counter = counter.clone();
        async move {
            let num = counter.load(Ordering::Relaxed);
            if num > 1 {
                Ok(true)
            } else {
                counter.store(num + 1, Ordering::Relaxed);
                Err("nope")
            }
        }
    })
    .await?;
    Ok(())
}
