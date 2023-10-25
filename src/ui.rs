use std::{
    collections::HashMap,
    iter::FromIterator,
    sync::{Arc, Mutex},
};

use sciter::Value;

use hbb_common::{
    allow_err,
    config::{LocalConfig, PeerConfig},
    log,
};

#[cfg(not(any(feature = "flutter", feature = "cli")))]
use crate::ui_session_interface::Session;
use crate::{common::get_app_name, ipc, ui_interface::*};

mod cm;
#[cfg(feature = "inline")]
pub mod inline;
pub mod remote;

#[allow(dead_code)]
type Status = (i32, bool, i64, String);

lazy_static::lazy_static! {
    // stupid workaround for https://sciter.com/forums/topic/crash-on-latest-tis-mac-sdk-sometimes/
    static ref STUPID_VALUES: Mutex<Vec<Arc<Vec<Value>>>> = Default::default();
}

#[cfg(not(any(feature = "flutter", feature = "cli")))]
lazy_static::lazy_static! {
    pub static ref CUR_SESSION: Arc<Mutex<Option<Session<remote::SciterHandler>>>> = Default::default();
}

struct UIHostHandler;

pub fn start(args: &mut [String]) {
    #[cfg(target_os = "macos")]
    crate::platform::delegate::show_dock();
    #[cfg(all(target_os = "linux", feature = "inline"))]
    {
        #[cfg(feature = "appimage")]
        let prefix = std::env::var("APPDIR").unwrap_or("".to_string());
        #[cfg(not(feature = "appimage"))]
        let prefix = "".to_string();
        #[cfg(feature = "flatpak")]
        let dir = "/app";
        #[cfg(not(feature = "flatpak"))]
        let dir = "/usr";
        sciter::set_library(&(prefix + dir + "/lib/dshelpdesk/libsciter-gtk.so")).ok();
    }
    #[cfg(windows)]
    // Check if there is a sciter.dll nearby.
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let sciter_dll_path = parent.join("sciter.dll");
            if sciter_dll_path.exists() {
                // Try to set the sciter dll.
                let p = sciter_dll_path.to_string_lossy().to_string();
                log::debug!("Found dll:{}, \n {:?}", p, sciter::set_library(&p));
            }
        }
    }
    // https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-types.h
    // https://github.com/rustdesk/dshelpdesk/issues/132#issuecomment-886069737
    #[cfg(windows)]
    allow_err!(sciter::set_options(sciter::RuntimeOptions::GfxLayer(
        sciter::GFX_LAYER::WARP
    )));
    use sciter::SCRIPT_RUNTIME_FEATURES::*;
    allow_err!(sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
        ALLOW_FILE_IO as u8 | ALLOW_SOCKET_IO as u8 | ALLOW_EVAL as u8 | ALLOW_SYSINFO as u8
    )));
    let mut frame = sciter::WindowBuilder::main_window().create();
    #[cfg(windows)]
    allow_err!(sciter::set_options(sciter::RuntimeOptions::UxTheming(true)));
    frame.set_title(&crate::get_app_name());
    #[cfg(target_os = "macos")]
    crate::platform::delegate::make_menubar(frame.get_host(), args.is_empty());
    let page;
    if args.len() > 1 && args[0] == "--play" {
        args[0] = "--connect".to_owned();
        let path: std::path::PathBuf = (&args[1]).into();
        let id = path
            .file_stem()
            .map(|p| p.to_str().unwrap_or(""))
            .unwrap_or("")
            .to_owned();
        args[1] = id;
    }
    if args.is_empty() {
        std::thread::spawn(move || check_zombie());
        crate::common::check_software_update();
        frame.event_handler(UI {});
        frame.sciter_handler(UIHostHandler {});
        page = "index.html";
        // Start pulse audio local server.
        #[cfg(target_os = "linux")]
        std::thread::spawn(crate::ipc::start_pa);
    } else if args[0] == "--install" {
        frame.event_handler(UI {});
        frame.sciter_handler(UIHostHandler {});
        page = "install.html";
    } else if args[0] == "--cm" {
        frame.register_behavior("connection-manager", move || {
            Box::new(cm::SciterConnectionManager::new())
        });
        page = "cm.html";
    } else if (args[0] == "--connect"
        || args[0] == "--file-transfer"
        || args[0] == "--port-forward"
        || args[0] == "--rdp")
        && args.len() > 1
    {
        #[cfg(windows)]
        {
            let hw = frame.get_host().get_hwnd();
            crate::platform::windows::enable_lowlevel_keyboard(hw as _);
        }
        let mut iter = args.iter();
        let Some(cmd) = iter.next() else {
            log::error!("Failed to get cmd arg");
            return;
        };
        let cmd = cmd.to_owned();
        let Some(id) = iter.next() else {
            log::error!("Failed to get id arg");
            return;
        };
        let id = id.to_owned();
        let pass = iter.next().unwrap_or(&"".to_owned()).clone();
        let args: Vec<String> = iter.map(|x| x.clone()).collect();
        frame.set_title(&id);
        frame.register_behavior("native-remote", move || {
            let handler =
                remote::SciterSession::new(cmd.clone(), id.clone(), pass.clone(), args.clone());
            #[cfg(not(any(feature = "flutter", feature = "cli")))]
            {
                *CUR_SESSION.lock().unwrap() = Some(handler.inner());
            }
            Box::new(handler)
        });
        page = "remote.html";
    } else {
        log::error!("Wrong command: {:?}", args);
        return;
    }
    #[cfg(feature = "inline")]
    {
        let html = if page == "index.html" {
            inline::get_index()
        } else if page == "cm.html" {
            inline::get_cm()
        } else if page == "install.html" {
            inline::get_install()
        } else {
            inline::get_remote()
        };
        frame.load_html(html.as_bytes(), Some(page));
    }
    #[cfg(not(feature = "inline"))]
    frame.load_file(&format!(
        "file://{}/src/ui/{}",
        std::env::current_dir()
            .map(|c| c.display().to_string())
            .unwrap_or("".to_owned()),
        page
    ));
    frame.run_app();
}

struct UI {}

impl UI {
    fn recent_sessions_updated(&self) -> bool {
        recent_sessions_updated()
    }

    fn get_id(&self) -> String {
        ipc::get_id()
    }

    fn temporary_password(&mut self) -> String {
        temporary_password()
    }

    fn update_temporary_password(&self) {
        update_temporary_password()
    }

    fn permanent_password(&self) -> String {
        permanent_password()
    }

    fn set_permanent_password(&self, password: String) {
        set_permanent_password(password);
    }

    fn get_remote_id(&mut self) -> String {
        LocalConfig::get_remote_id()
    }

    fn set_remote_id(&mut self, id: String) {
        LocalConfig::set_remote_id(&id);
    }

    fn goto_install(&mut self) {
        goto_install();
    }

    fn install_me(&mut self, _options: String, _path: String) {
        install_me(_options, _path, false, false);
    }

    fn update_me(&self, _path: String) {
        update_me(_path);
    }

    fn run_without_install(&self) {
        run_without_install();
    }

    fn show_run_without_install(&self) -> bool {
        show_run_without_install()
    }

    fn get_license(&self) -> String {
        get_license()
    }

    fn get_option(&self, key: String) -> String {
        get_option(key)
    }

    fn get_local_option(&self, key: String) -> String {
        get_local_option(key)
    }

    fn set_local_option(&self, key: String, value: String) {
        set_local_option(key, value);
    }

    fn peer_has_password(&self, id: String) -> bool {
        peer_has_password(id)
    }

    fn forget_password(&self, id: String) {
        forget_password(id)
    }

    fn get_peer_option(&self, id: String, name: String) -> String {
        get_peer_option(id, name)
    }

    fn set_peer_option(&self, id: String, name: String, value: String) {
        set_peer_option(id, name, value)
    }

    fn using_public_server(&self) -> bool {
        using_public_server()
    }

    fn get_options(&self) -> Value {
        let hashmap: HashMap<String, String> =
            serde_json::from_str(&get_options()).unwrap_or_default();
        let mut m = Value::map();
        for (k, v) in hashmap {
            m.set_item(k, v);
        }
        m
    }

    fn test_if_valid_server(&self, host: String) -> String {
        test_if_valid_server(host)
    }

    fn get_sound_inputs(&self) -> Value {
        Value::from_iter(get_sound_inputs())
    }

    fn set_options(&self, v: Value) {
        let mut m = HashMap::new();
        for (k, v) in v.items() {
            if let Some(k) = k.as_string() {
                if let Some(v) = v.as_string() {
                    if !v.is_empty() {
                        m.insert(k, v);
                    }
                }
            }
        }
        set_options(m);
    }

    fn set_option(&self, key: String, value: String) {
        set_option(key, value);
    }

    fn install_path(&mut self) -> String {
        install_path()
    }

    fn get_socks(&self) -> Value {
        Value::from_iter(get_socks())
    }

    fn set_socks(&self, proxy: String, username: String, password: String) {
        set_socks(proxy, username, password)
    }

    fn is_installed(&self) -> bool {
        is_installed()
    }

    fn is_root(&self) -> bool {
        is_root()
    }

    fn is_release(&self) -> bool {
        #[cfg(not(debug_assertions))]
        return true;
        #[cfg(debug_assertions)]
        return false;
    }

    fn is_rdp_service_open(&self) -> bool {
        is_rdp_service_open()
    }

    fn is_share_rdp(&self) -> bool {
        is_share_rdp()
    }

    fn set_share_rdp(&self, _enable: bool) {
        set_share_rdp(_enable);
    }

    fn is_installed_lower_version(&self) -> bool {
        is_installed_lower_version()
    }

    fn closing(&mut self, x: i32, y: i32, w: i32, h: i32) {
        crate::server::input_service::fix_key_down_timeout_at_exit();
        LocalConfig::set_size(x, y, w, h);
    }

    fn get_size(&mut self) -> Value {
        let s = LocalConfig::get_size();
        let mut v = Vec::new();
        v.push(s.0);
        v.push(s.1);
        v.push(s.2);
        v.push(s.3);
        Value::from_iter(v)
    }

    fn get_mouse_time(&self) -> f64 {
        get_mouse_time()
    }

    fn check_mouse_time(&self) {
        check_mouse_time()
    }

    fn get_connect_status(&mut self) -> Value {
        let mut v = Value::array(0);
        let x = get_connect_status();
        v.push(x.status_num);
        v.push(x.key_confirmed);
        v.push(x.id);
        v
    }

    #[inline]
    fn get_peer_value(id: String, p: PeerConfig) -> Value {
        let values = vec![
            id,
            p.info.username.clone(),
            p.info.hostname.clone(),
            p.info.platform.clone(),
            p.options.get("alias").unwrap_or(&"".to_owned()).to_owned(),
        ];
        Value::from_iter(values)
    }

    fn get_peer(&self, id: String) -> Value {
        let c = get_peer(id.clone());
        Self::get_peer_value(id, c)
    }

    fn get_fav(&self) -> Value {
        Value::from_iter(get_fav())
    }

    fn store_fav(&self, fav: Value) {
        let mut tmp = vec![];
        fav.values().for_each(|v| {
            if let Some(v) = v.as_string() {
                if !v.is_empty() {
                    tmp.push(v);
                }
            }
        });
        store_fav(tmp);
    }

    fn get_recent_sessions(&mut self) -> Value {
        // to-do: limit number of recent sessions, and remove old peer file
        let peers: Vec<Value> = PeerConfig::peers(None)
            .drain(..)
            .map(|p| Self::get_peer_value(p.0, p.2))
            .collect();
        Value::from_iter(peers)
    }

    fn get_icon(&mut self) -> String {
        get_icon()
    }

    fn remove_peer(&mut self, id: String) {
        PeerConfig::remove(&id);
    }

    fn remove_discovered(&mut self, id: String) {
        remove_discovered(id);
    }

    fn send_wol(&mut self, id: String) {
        crate::lan::send_wol(id)
    }

    fn new_remote(&mut self, id: String, remote_type: String, force_relay: bool) {
        new_remote(id, remote_type, force_relay)
    }

    fn is_process_trusted(&mut self, _prompt: bool) -> bool {
        is_process_trusted(_prompt)
    }

    fn is_can_screen_recording(&mut self, _prompt: bool) -> bool {
        is_can_screen_recording(_prompt)
    }

    fn is_installed_daemon(&mut self, _prompt: bool) -> bool {
        is_installed_daemon(_prompt)
    }

    fn get_error(&mut self) -> String {
        get_error()
    }

    fn is_login_wayland(&mut self) -> bool {
        is_login_wayland()
    }

    fn current_is_wayland(&mut self) -> bool {
        current_is_wayland()
    }

    fn get_software_update_url(&self) -> String {
        crate::SOFTWARE_UPDATE_URL.lock().unwrap().clone()
    }

    fn get_new_version(&self) -> String {
        get_new_version()
    }

    fn get_version(&self) -> String {
        get_version()
    }

    fn get_fingerprint(&self) -> String {
        get_fingerprint()
    }

    fn get_app_name(&self) -> String {
        get_app_name()
    }

    fn get_software_ext(&self) -> String {
        #[cfg(windows)]
        let p = "exe";
        #[cfg(target_os = "macos")]
        let p = "dmg";
        #[cfg(target_os = "linux")]
        let p = "deb";
        p.to_owned()
    }

    fn get_software_store_path(&self) -> String {
        let mut p = std::env::temp_dir();
        let name = crate::SOFTWARE_UPDATE_URL
            .lock()
            .unwrap()
            .split("/")
            .last()
            .map(|x| x.to_owned())
            .unwrap_or(crate::get_app_name());
        p.push(name);
        format!("{}.{}", p.to_string_lossy(), self.get_software_ext())
    }

    fn create_shortcut(&self, _id: String) {
        #[cfg(windows)]
        create_shortcut(_id)
    }

    fn discover(&self) {
        std::thread::spawn(move || {
            allow_err!(crate::lan::discover());
        });
    }

    fn get_lan_peers(&self) -> String {
        // let peers = get_lan_peers()
        //     .into_iter()
        //     .map(|mut peer| {
        //         (
        //             peer.remove("id").unwrap_or_default(),
        //             peer.remove("username").unwrap_or_default(),
        //             peer.remove("hostname").unwrap_or_default(),
        //             peer.remove("platform").unwrap_or_default(),
        //         )
        //     })
        //     .collect::<Vec<(String, String, String, String)>>();
        serde_json::to_string(&get_lan_peers()).unwrap_or_default()
    }

    fn get_uuid(&self) -> String {
        get_uuid()
    }

    fn open_url(&self, url: String) {
        #[cfg(windows)]
        let p = "explorer";
        #[cfg(target_os = "macos")]
        let p = "open";
        #[cfg(target_os = "linux")]
        let p = if std::path::Path::new("/usr/bin/firefox").exists() {
            "firefox"
        } else {
            "xdg-open"
        };
        allow_err!(std::process::Command::new(p).arg(url).spawn());
    }

    fn change_id(&self, id: String) {
        reset_async_job_status();
        let old_id = self.get_id();
        change_id_shared(id, old_id);
    }

    fn post_request(&self, url: String, body: String, header: String) {
        post_request(url, body, header)
    }

    fn is_ok_change_id(&self) -> bool {
        hbb_common::machine_uid::get().is_ok()
    }

    fn get_async_job_status(&self) -> String {
        get_async_job_status()
    }

    fn t(&self, name: String) -> String {
        crate::client::translate(name)
    }

    fn is_xfce(&self) -> bool {
        crate::platform::is_xfce()
    }

    fn get_api_server(&self) -> String {
        get_api_server()
    }

    fn has_hwcodec(&self) -> bool {
        has_hwcodec()
    }

    fn get_langs(&self) -> String {
        get_langs()
    }

    fn default_video_save_directory(&self) -> String {
        default_video_save_directory()
    }

    fn handle_relay_id(&self, id: String) -> String {
        handle_relay_id(id)
    }

    fn get_login_device_info(&self) -> String {
        get_login_device_info_json()
    }

    fn support_remove_wallpaper(&self) -> bool {
        support_remove_wallpaper()
    }
}

impl sciter::EventHandler for UI {
    sciter::dispatch_script_call! {
        fn t(String);
        fn get_api_server();
        fn is_xfce();
        fn using_public_server();
        fn get_id();
        fn temporary_password();
        fn update_temporary_password();
        fn permanent_password();
        fn set_permanent_password(String);
        fn get_remote_id();
        fn set_remote_id(String);
        fn closing(i32, i32, i32, i32);
        fn get_size();
        fn new_remote(String, String, bool);
        fn send_wol(String);
        fn remove_peer(String);
        fn remove_discovered(String);
        fn get_connect_status();
        fn get_mouse_time();
        fn check_mouse_time();
        fn get_recent_sessions();
        fn get_peer(String);
        fn get_fav();
        fn store_fav(Value);
        fn recent_sessions_updated();
        fn get_icon();
        fn install_me(String, String);
        fn is_installed();
        fn is_root();
        fn is_release();
        fn set_socks(String, String, String);
        fn get_socks();
        fn is_rdp_service_open();
        fn is_share_rdp();
        fn set_share_rdp(bool);
        fn is_installed_lower_version();
        fn install_path();
        fn goto_install();
        fn is_process_trusted(bool);
        fn is_can_screen_recording(bool);
        fn is_installed_daemon(bool);
        fn get_error();
        fn is_login_wayland();
        fn current_is_wayland();
        fn get_options();
        fn get_custom_options();
        fn get_option(String);
        fn get_local_option(String);
        fn set_local_option(String, String);
        fn get_peer_option(String, String);
        fn peer_has_password(String);
        fn forget_password(String);
        fn set_peer_option(String, String, String);
        fn get_license();
        fn test_if_valid_server(String);
        fn get_sound_inputs();
        fn set_options(Value);
        fn set_option(String, String);
        fn get_software_update_url();
        fn get_new_version();
        fn get_version();
        fn get_fingerprint();
        fn update_me(String);
        fn show_run_without_install();
        fn run_without_install();
        fn get_app_name();
        fn get_software_store_path();
        fn get_software_ext();
        fn open_url(String);
        fn change_id(String);
        fn get_async_job_status();
        fn post_request(String, String, String);
        fn is_ok_change_id();
        fn create_shortcut(String);
        fn discover();
        fn get_lan_peers();
        fn get_uuid();
        fn has_hwcodec();
        fn get_langs();
        fn default_video_save_directory();
        fn handle_relay_id(String);
        fn get_login_device_info();
        fn support_remove_wallpaper();
    }
}

impl sciter::host::HostHandler for UIHostHandler {
    fn on_graphics_critical_failure(&mut self) {
        log::error!("Critical rendering error: e.g. DirectX gfx driver error. Most probably bad gfx drivers.");
    }
}

#[cfg(not(target_os = "linux"))]
fn get_sound_inputs() -> Vec<String> {
    let mut out = Vec::new();
    use cpal::traits::{DeviceTrait, HostTrait};
    let host = cpal::default_host();
    if let Ok(devices) = host.devices() {
        for device in devices {
            if device.default_input_config().is_err() {
                continue;
            }
            if let Ok(name) = device.name() {
                out.push(name);
            }
        }
    }
    out
}

#[cfg(target_os = "linux")]
fn get_sound_inputs() -> Vec<String> {
    crate::platform::linux::get_pa_sources()
        .drain(..)
        .map(|x| x.1)
        .collect()
}

// sacrifice some memory
pub fn value_crash_workaround(values: &[Value]) -> Arc<Vec<Value>> {
    let persist = Arc::new(values.to_vec());
    STUPID_VALUES.lock().unwrap().push(persist.clone());
    persist
}

pub fn get_icon() -> String {
    // 128x128
    #[cfg(target_os = "macos")]
    // 128x128 on 160x160 canvas, then shrink to 128, mac looks better with padding
    {
        "data:image/png;base64,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".into()
    }
    #[cfg(not(target_os = "macos"))] // 128x128 no padding
    {
        "data:image/png;base64,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".into()
    }
}
