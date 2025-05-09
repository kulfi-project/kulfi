#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn ui() -> eyre::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .register_asynchronous_uri_scheme_protocol("kulfi", |_ctx, request, _responder| {
            tauri::async_runtime::spawn(async move {
                let _request = kulfi_utils::http::vec_u8_to_bytes(request);
                // responder.respond(kulfi_utils::http::response_to_static(Ok(
                //     kulfi_utils::http_to_peer(request).await.unwrap(),
                // )))
                todo!()
            });
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
