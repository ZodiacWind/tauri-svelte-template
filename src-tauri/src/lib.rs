
use tauri::{
	webview::{PageLoadEvent, WebviewWindowBuilder},
	App, Emitter, Listener, Runtime, WebviewUrl,
};
#[allow(unused)]
use tauri::{Manager, RunEvent};
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	run_app(tauri::Builder::default(), |_app| {})
}

pub fn run_app<R: Runtime, F: FnOnce(&App<R>) + Send + 'static>(
	builder: tauri::Builder<R>,
	setup: F,
) {
	#[allow(unused_mut)]
	let mut builder = builder
		.setup(move |app| {

			let mut window_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());

			#[cfg(all(desktop, not(test)))]
			{
				window_builder = window_builder
					.title("Tauri SvelteKit Template")
					.inner_size(1000., 800.)
					.min_inner_size(600., 400.)
					.menu(tauri::menu::Menu::default(app.handle())?);
			}
			window_builder.build()?;
			setup(app);

			Ok(())
		});

	#[allow(unused_mut)]
	let mut app = builder
		.invoke_handler(tauri::generate_handler![
    ])
		.build(tauri::generate_context!())
		.expect("error while building tauri application");

	app.run(move |_app_handle, _event| {
		#[cfg(all(desktop, not(test)))]
		match &_event {
			RunEvent::ExitRequested { api, code, .. } => {
				// Keep the event loop running even if all windows are closed
				// This allow us to catch tray icon events when there is no window
				// if we manually requested an exit (code is Some(_)) we will let it go through
				// if code.is_none() {
				// 	api.prevent_exit();
				// }
			}
			RunEvent::WindowEvent {
				event: tauri::WindowEvent::CloseRequested { api, .. },
				label,
				..
			} => {
				println!("closing window...");
				// run the window destroy manually just for fun :)
				// usually you'd show a dialog here to ask for confirmation or whatever
				// api.prevent_close();
				// _app_handle
				// 	.get_webview_window(label)
				// 	.unwrap()
				// 	.destroy()
				// 	.unwrap();
			}
			_ => (),
		}
	})
}
