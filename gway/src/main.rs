use aok::{OK, Void};

#[global_allocator]
static MALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

async fn main() -> Void {
  log_init::init();
  xboot::init().await?;
  OK
}
