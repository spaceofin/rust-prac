use axum::{Router, routing::post};
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;
use std::fs;

static OUTPUT_DIR: Lazy<PathBuf> = Lazy::new(|| {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src/routes/output")
});

async fn save_txt_file(input: String) -> String {
  let file_path = OUTPUT_DIR.join("txt_output.txt");

  if let Err(e) = fs::write(&file_path, input) {
    return format!("Failed to write file:\n{}", e);
  }

  format!("File saved successfully:\n{}", file_path.display())
}

pub fn create_router() -> Router {
  fs::create_dir_all(&*OUTPUT_DIR).expect("Failed to create output directory");

  Router::new()
    .nest("/files",
      Router::new()
        .route("/txt", post(save_txt_file))
  )
}