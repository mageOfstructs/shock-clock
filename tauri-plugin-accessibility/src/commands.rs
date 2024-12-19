use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::AccessibilityExt;
use crate::Result;

// #[command]
// pub(crate) async fn ping<R: Runtime>(
//     app: AppHandle<R>,
//     payload: PingRequest,
// ) -> Result<PingResponse> {
//     app.accessibility().ping(payload)
// }
// #[derive(Serialize, Deserialize)]
// struct EventPayload;
//
// #[command]
// pub(crate) async fn get_event<R: Runtime>(
//     app: AppHandle<R>,
//     payload: EventPayload,
// ) -> Result<PingResponse> {
//     app.accessibility()(payload)
// }
