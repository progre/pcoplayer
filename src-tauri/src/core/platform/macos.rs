use std::{fs::OpenOptions, io::Write, mem, sync::OnceLock};

use objc2::{
    declare_class, msg_send_id, mutability::MainThreadOnly, rc::Retained, runtime::ProtocolObject,
    ClassType, DeclaredClass,
};
use objc2_app_kit::{NSApplication, NSApplicationDelegate};
use objc2_foundation::{MainThreadMarker, NSArray, NSObject, NSObjectProtocol, NSURL};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use uuid::Uuid;

use crate::core::state::{state, StreamInfo};

fn log_to_file(message: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("debug.log")
        .unwrap();
    file.write_fmt(format_args!("{}\n", message)).unwrap();
}

fn create_window(app_handle: &AppHandle) -> WebviewWindow {
    let webview_url = WebviewUrl::App("index.html".into());
    WebviewWindowBuilder::new(app_handle, Uuid::new_v4().to_string(), webview_url)
        .title("pcoplayer")
        .build()
        .unwrap()
}

fn application_open_urls(
    _zelf: &AppDelegate,
    _application: &NSApplication,
    urls: &NSArray<NSURL>,
) -> bool {
    let url = urls.to_vec()[0];
    let url_string = unsafe { url.absoluteString() }.unwrap().to_string();

    let app = &PLATFORM.get().unwrap().app;
    let window = create_window(app);

    let state = state(window.app_handle());
    let mut state = state.lock().unwrap();
    state.init_window(
        &window,
        StreamInfo {
            url: Some(url_string),
            channel_name: None,
            contact_url: None,
        },
    );
    true
}

pub struct Ivars {}

declare_class!(
    struct AppDelegate;

    unsafe impl ClassType for AppDelegate {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
        const NAME: &'static str = "AppDelegate";
    }

    impl DeclaredClass for AppDelegate {
        type Ivars = Ivars;
    }

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl NSApplicationDelegate for AppDelegate {
        #[method(application:openURLs:)]
        unsafe fn application_open_urls(&self, application: &NSApplication, urls: &NSArray<NSURL>) -> bool {
            application_open_urls(self, application, urls)
        }
    }
);

impl AppDelegate {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = mtm.alloc();
        let this = this.set_ivars(Ivars {});
        unsafe { msg_send_id![super(this), init] }
    }
}

static PLATFORM: OnceLock<Platform> = OnceLock::new();

#[derive(Debug)]
struct Platform {
    app: AppHandle,
}

pub fn setup(app: AppHandle) {
    let mtm = MainThreadMarker::new().unwrap();
    let app_delegate = AppDelegate::new(mtm);
    let application = NSApplication::sharedApplication(mtm);
    application.setDelegate(Some(ProtocolObject::from_ref(&*app_delegate)));
    mem::forget(app_delegate);
    let zelf = Platform { app };
    PLATFORM.set(zelf).unwrap();
}
