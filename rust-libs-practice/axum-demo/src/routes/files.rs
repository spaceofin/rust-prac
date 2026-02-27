use axum::{Router, routing::post};
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;
use std::fs;

static OUTPUT_DIR: Lazy<PathBuf> = Lazy::new(|| {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src/routes/output")
});

async fn save_txt_file(body: String) -> String {
  let file_path = OUTPUT_DIR.join("txt_output.txt");

  if let Err(e) = fs::write(&file_path, body) {
    return format!("Failed to write file:\n{}", e);
  }

  format!("File saved successfully:\n{}", file_path.display())
}

use axum::extract::Multipart;

async fn read_multipart(mut body: Multipart) -> Result<String, String> {
  let mut output = String::new();

  while let Some(field) = body.next_field().await.map_err(|e| e.to_string())? {
    let name = field.name().filter(|s| !s.is_empty()).unwrap_or("unnamed").to_string();
    let data = field.bytes().await.map_err(|e| e.to_string())?;
    let preview = &data[..data.len().min(10)];
    output.push_str(&format!("{} : {} bytes, preview: {:?}\n", name, data.len(), preview));
  }

  if output.is_empty() {
    Err("No fields found in multipart data".to_string())
  } else {
    Ok(output)
  }
}

pub fn create_router() -> Router {
  fs::create_dir_all(&*OUTPUT_DIR).expect("Failed to create output directory");

  Router::new()
    .nest("/files",
      Router::new()
        .route("/txt", post(save_txt_file))
        .route("/multipart", post(read_multipart))
  )
}