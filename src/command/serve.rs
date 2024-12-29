use crate::{
    services::serialization,
    state::AppState,
    util::{print_not_initialized, read_ris_files_from_dir},
};
use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};
use colored::Colorize;
use serde::Deserialize;
use std::net::SocketAddr;

/// Shared state for all handlers.
/// You can store additional fields as needed.
#[derive(Clone)]
struct AppData {
    project_path: String,
}

/// GET /
/// Show the list of references from ris_folder, with an "Edit" button for each item,
/// plus "Upload" and "Update" buttons at the top.
async fn index_handler(
    State(app_data): State<AppData>,
) -> Result<Html<String>, (StatusCode, String)> {
    let ris_entries = read_ris_files_from_dir(&format!("{}/ris_files", app_data.project_path))
        .unwrap_or_default();

    // Start building the HTML.
    // This page has:
    // 1) "Upload File" button that goes to /upload
    // 2) "Update" button that sends POST to /update
    // 3) Table of references with "Edit" button linking to /edit/<some_id>

    let mut html = String::new();
    html.push_str(
        r#"
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>RIS / BibTeX Viewer</title>
  <script src="https://cdn.tailwindcss.com"></script>
        </head>
        <body class="bg-gray-900 text-gray-100 min-h-screen">
            <header class="p-4 bg-gray-800 shadow-md mb-6">
                <h1 class="text-2xl font-bold text-center tracking-wider">Reference Tracker</h1>
                <p class="text-center text-gray-400 text-sm mb-4">Manage your .ris &amp; .bib files in one place</p>
                <div class="flex justify-center gap-4">
                    <a href="/add" class="bg-orange-600 hover:bg-orange-700 text-white py-2 px-4 rounded">Add RIS/BibTeX</a>
                    <a href="/upload" class="bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded">Upload File</a>
                    <form action="/update" method="post">
                        <button type="submit" class="bg-green-600 hover:bg-green-700 text-white py-2 px-4 rounded">
                            Update
                        </button>
                    </form>
                </div>
            </header>

            <main class="max-w-6xl mx-auto px-4">
                <section class="mb-6">
                    <h2 class="text-xl font-semibold border-b border-gray-700 pb-2 mb-4">RIS File Table</h2>
                    <div class="overflow-x-auto rounded-lg shadow-lg">
                        <table class="min-w-full border-collapse">
                            <thead class="bg-gray-800 border-b border-gray-700">
                                <tr>
                                    <th class="px-4 py-3 text-left font-medium uppercase tracking-wider text-gray-200">Author</th>
                                    <th class="px-4 py-3 text-left font-medium uppercase tracking-wider text-gray-200">Title</th>
                                    <th class="px-4 py-3 text-left font-medium uppercase tracking-wider text-gray-200">Year</th>
                                    <th class="px-4 py-3 text-left font-medium uppercase tracking-wider text-gray-200">Actions</th>
                                </tr>
                            </thead>
                            <tbody>
    "#
    );

    // Populate the table rows. We'll pretend "Edit" uses some ID. You can generate IDs as needed.
    for (i, entry) in ris_entries.iter().enumerate() {
        let author = entry
            .fields
            .get("AU")
            .map(|authors| authors.join(", "))
            .unwrap_or_else(|| "Unknown".to_string());
        let title = entry
            .fields
            .get("TI")
            .and_then(|titles| titles.first().cloned())
            .unwrap_or_else(|| "Unknown".to_string());
        let year = entry
            .fields
            .get("PY")
            .and_then(|years| years.first().cloned())
            .unwrap_or_else(|| "Unknown".to_string());

        // We'll use `i` as a placeholder ID. If you have an actual unique ID in your data, use that.
        html.push_str(&format!(
            r#"
                                <tr class="border-b border-gray-700 hover:bg-gray-800 transition-colors">
                                    <td class="px-4 py-3 align-top">{author}</td>
                                    <td class="px-4 py-3 align-top">{title}</td>
                                    <td class="px-4 py-3 align-top">{year}</td>
                                    <td class="px-4 py-3 align-top">
                                        <a href="/edit/{i}" class="bg-purple-600 hover:bg-purple-700 text-white px-3 py-1 rounded">
                                            Edit
                                        </a>
                                    </td>
                                </tr>
            "#
        ));
    }

    html.push_str(
        r#"
                            </tbody>
                        </table>
                    </div>
                </section>
            </main>

            <footer class="bg-gray-800 p-4 text-center text-sm text-gray-500 mt-auto">
                <p>© 2024 Reference Tracker. All rights reserved.</p>
            </footer>
        </body>
        </html>
    "#,
    );

    Ok(Html(html))
}

/// GET /upload
/// A simple page with a placeholder form for uploading a new reference file.
async fn upload_handler() -> Html<String> {
    let html = r#"
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Upload File</title>
              <script src="https://cdn.tailwindcss.com"></script>
        </head>
        <body class="bg-gray-900 text-gray-100 min-h-screen">
            <header class="p-4 bg-gray-800 shadow-md mb-6">
                <h1 class="text-2xl font-bold text-center tracking-wider">Reference Tracker - Upload</h1>
            </header>
            <main class="max-w-lg mx-auto px-4">
                <form action="/upload" method="post" enctype="multipart/form-data" class="bg-gray-800 p-4 rounded shadow">
                    <label class="block mb-2 font-medium" for="file">Select a file to upload:</label>
                    <input class="mb-4 block w-full text-sm text-gray-200 file:mr-4 file:py-2 file:px-4
                              file:rounded file:border-0
                              file:text-sm file:font-semibold
                              file:bg-purple-600 file:text-white
                              hover:file:bg-purple-700
                              " type="file" id="file" name="file" required />
                    <button class="bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded"
                            type="submit">Upload</button>
                </form>
            </main>
        </body>
        </html>
    "#;
    Html(html.to_string())
}

/// POST /upload
/// A placeholder for actually processing the uploaded file.
async fn upload_post_handler() -> impl IntoResponse {
    // TODO: Implement file handling logic here
    // e.g., store the uploaded file in `ris_folder`, parse it, etc.
    Html(r#"<p class="text-white">File uploaded successfully (placeholder)!</p>"#)
}

/// GET /edit/:id
/// A simple page for editing an existing reference, identified by :id.
async fn edit_handler(Path(id): Path<usize>) -> Html<String> {
    // In a real app, you'd load the reference from the database or memory using `id`.
    // Then you’d generate a form pre-filled with that reference's data.
    // For now, we’ll just have a placeholder form.
    let html = format!(
        r#"
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Edit Reference</title>
  <script src="https://cdn.tailwindcss.com"></script>
        </head>
        <body class="bg-gray-900 text-gray-100 min-h-screen">
            <header class="p-4 bg-gray-800 shadow-md mb-6">
                <h1 class="text-2xl font-bold text-center tracking-wider">Editing Reference #{id}</h1>
            </header>
            <main class="max-w-lg mx-auto px-4">
                <form action="/edit/{id}" method="post" class="bg-gray-800 p-4 rounded shadow">
                    <label class="block mb-2 font-medium" for="author">Author:</label>
                    <input id="author" name="author" class="mb-4 block w-full text-gray-200 bg-gray-700 p-2 rounded" value="Doe, John" />

                    <label class="block mb-2 font-medium" for="title">Title:</label>
                    <input id="title" name="title" class="mb-4 block w-full text-gray-200 bg-gray-700 p-2 rounded" value="Placeholder Title" />

                    <label class="block mb-2 font-medium" for="year">Year:</label>
                    <input id="year" name="year" class="mb-4 block w-full text-gray-200 bg-gray-700 p-2 rounded" value="2024" />

                    <button class="bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded" type="submit">
                        Save
                    </button>
                </form>
            </main>
        </body>
        </html>
    "#
    );
    Html(html)
}

/// POST /edit/:id
/// A placeholder for saving changes to the reference.
async fn edit_post_handler(Path(id): Path<usize>) -> impl IntoResponse {
    // TODO: Implement actual "edit reference" logic
    // e.g., parse form, update .ris file or database, etc.
    Html(format!(
        r#"<p class="text-white">Reference #{} updated successfully (placeholder)!</p>"#,
        id
    ))
}

/// POST /update
/// Calls logic to "sync with the cloud" or otherwise update references externally.
async fn update_handler() -> impl IntoResponse {
    // TODO: Implement the actual sync logic
    // e.g., push local .ris data to remote server, handle merges, etc.
    Html(r#"<p class="text-white">Updated/synced with the cloud (placeholder)!</p>"#)
}

/// GET /add
/// Shows a page with a textarea for pasting RIS or BibTeX content.
async fn add_ris_bibtex_handler() -> Html<String> {
    let html = r#"
        <html lang="en">
        <head>
            <meta charset="UTF-8" />
            <title>Add RIS/BibTeX</title>
            <script src="https://cdn.tailwindcss.com"></script>
        </head>
        <body class="bg-gray-900 text-gray-100 min-h-screen">
            <header class="p-4 bg-gray-800 shadow-md mb-6">
                <h1 class="text-2xl font-bold text-center tracking-wider">Add RIS/BibTeX References</h1>
            </header>

            <main class="max-w-lg mx-auto px-4">
                <form action="/add" method="post" class="bg-gray-800 p-4 rounded shadow">
                    <label class="block mb-2 font-medium" for="references">Paste RIS or BibTeX data here:</label>
                    <textarea
                        id="references"
                        name="references"
                        rows="10"
                        class="w-full text-gray-200 bg-gray-700 p-2 rounded mb-4"
                        placeholder="Paste your RIS or BibTeX entries here..."></textarea>

                    <button
                        class="bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded"
                        type="submit">
                        Add
                    </button>
                </form>
            </main>
        </body>
        </html>
    "#;

    Html(html.to_string())
}

#[derive(Deserialize)]
struct AddReferencesForm {
    references: String,
}

/// POST /add
/// Processes the pasted RIS/BibTeX data.
async fn add_ris_bibtex_post_handler(
    State(app_data): State<AppData>,
    Form(form): Form<AddReferencesForm>,
) -> impl IntoResponse {
    // The raw text the user pasted:
    let pasted_content = form.references;

    // Determine the user-facing message based on the import result.
    let message = match serialization::import(&pasted_content, &app_data.project_path) {
        Ok(result) => match result {
            serialization::ImportResult::BibtexImported => {
                "Recognized <b>BibTex</b> and imported successfully.".to_string()
            }
            serialization::ImportResult::BibtexError { error } => {
                format!("BibTeX error: {error}")
            }
            serialization::ImportResult::RisImported => {
                "Recognized <b>RIS</b> and imported successfully.".to_string()
            }
            serialization::ImportResult::RisError { error } => {
                format!("RIS error: {error}")
            }
            serialization::ImportResult::UnrecognizedFormat => {
                "Unrecognized format. Could not import the data.".to_string()
            }
        },
        Err(err) => {
            // Unknown error that should not happen; show full layout with error
            let body = format!(
                r#"
                    <div class="bg-red-800 p-4 rounded mb-4">
                        <p class="text-white font-bold">Unknown server error:</p>
                        <p class="text-red-100">{}</p>
                    </div>
                    <p>
                        <a href="/" class="bg-gray-600 hover:bg-gray-700 text-white py-2 px-4 rounded">
                            Back to index
                        </a>
                    </p>
                "#,
                err
            );
            return Html(render_layout("Error", &body));
        }
    };

    // If we got here, we have a successful or recognized-but-with-errors import.
    // Show the message and the original pasted content, plus a back button.
    let body = format!(
        r#"
            <div class="bg-gray-800 p-4 rounded mb-4">
                <p class="text-white">{message}</p>
                <p class="text-white mt-2">Received references:</p>
                <pre class="bg-gray-700 text-gray-200 p-2 mt-2 rounded whitespace-pre-wrap">{pasted_content}</pre>
            </div>
            <p>
                <a href="/" class="bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded">Back to index</a>
            </p>
        "#
    );

    Html(render_layout("Add References Result", &body))
}

pub fn handle_serve(state: &AppState) -> Result<()> {
    if !state.initialized {
        print_not_initialized();
        return Ok(());
    }

    if state.projects.is_empty() {
        println!("{}", "No project selected.".blue().bold());
        return Ok(());
    }

    // Use your existing logic for choosing the folder.
    let project_path = state.current_project.clone();
    let app_data = AppData { project_path };

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        // Build the router with our multiple routes
        let app = Router::new()
            // Index page (list references)
            .route("/", get(index_handler))
            // Upload page
            .route("/upload", get(upload_handler).post(upload_post_handler))
            // Add references (new)
            .route(
                "/add",
                get(add_ris_bibtex_handler).post(add_ris_bibtex_post_handler),
            )
            // Edit page
            .route("/edit/:id", get(edit_handler).post(edit_post_handler))
            // Update route
            .route("/update", post(update_handler))
            // Provide our shared state (ris_folder, etc.)
            .with_state(app_data);

        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        println!("Server running on http://{}", addr);

        // Optionally open the default browser
        if webbrowser::open(&format!("http://{}", addr)).is_err() {
            eprintln!(
                "Failed to open browser. Please visit http://{} manually.",
                addr
            );
        }

        // Run the server
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .map_err(|e| anyhow::anyhow!("Server error: {e}"))
    })
}

/// Helper to wrap content in a consistent HTML layout with header & footer.
fn render_layout(page_title: &str, main_content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <title>{page_title}</title>
    <script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-gray-900 text-gray-100 min-h-screen flex flex-col">
    <header class="p-4 bg-gray-800 shadow-md mb-6">
        <h1 class="text-2xl font-bold text-center tracking-wider">Reference Tracker - {page_title}</h1>
        <p class="text-center text-gray-400 text-sm mb-4">Manage your .ris &amp; .bib files in one place</p>
        <div class="flex justify-center gap-4">
            <a href="/add" class="bg-orange-600 hover:bg-orange-700 text-white py-2 px-4 rounded">Add RIS/BibTeX</a>
            <a href="/upload" class="bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded">Upload File</a>
            <form action="/update" method="post">
                <button type="submit" class="bg-green-600 hover:bg-green-700 text-white py-2 px-4 rounded">
                    Update
                </button>
            </form>
        </div>
    </header>

    <main class="max-w-4xl mx-auto w-full px-4 flex-grow">
        {main_content}
    </main>

    <footer class="bg-gray-800 p-4 text-center text-sm text-gray-500 mt-auto">
        <p>© 2024 Reference Tracker. All rights reserved.</p>
    </footer>
</body>
</html>
"#
    )
}
