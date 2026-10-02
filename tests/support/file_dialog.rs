// Loaded as unit tests so the SDL callback can remain private.
use super::*;

fn context() -> (*mut c_void, Receiver<FileDialogResult>) {
    let (sender, receiver) = mpsc::channel();
    let name = CString::new("PNG Image").unwrap();
    let pattern = CString::new("png").unwrap();
    let context = Box::new(FileDialogContext {
        sender,
        filter: sdl3_sys::dialog::SDL_DialogFileFilter {
            name: name.as_ptr(),
            pattern: pattern.as_ptr(),
        },
        _name: name,
        _pattern: pattern,
        location: None,
    });
    (Box::into_raw(context).cast(), receiver)
}

#[test]
fn selected_path_is_owned_after_callback_returns() {
    let (userdata, receiver) = context();
    let path = CString::new("/tmp/skin é.png").unwrap();
    let filelist = [path.as_ptr(), std::ptr::null()];
    // A platform may invoke the callback on a worker thread.
    let userdata = userdata as usize;
    let filelist = filelist.map(|ptr| ptr as usize);
    std::thread::spawn(move || {
        let filelist = filelist.map(|ptr| ptr as *const c_char);
        unsafe { open_file_dialog_callback(userdata as *mut c_void, filelist.as_ptr(), 0) };
    })
    .join()
    .unwrap();
    drop(path);
    assert_eq!(
        receiver.recv().unwrap(),
        Ok(Some(PathBuf::from("/tmp/skin é.png")))
    );
    assert!(matches!(
        receiver.try_recv(),
        Err(mpsc::TryRecvError::Disconnected)
    ));
}

#[test]
fn cancellation_returns_none() {
    let (userdata, receiver) = context();
    let filelist = [std::ptr::null()];
    unsafe { open_file_dialog_callback(userdata, filelist.as_ptr(), -1) };
    assert_eq!(receiver.recv().unwrap(), Ok(None));
}

#[test]
fn null_filelist_reports_an_error() {
    let (userdata, receiver) = context();
    unsafe { open_file_dialog_callback(userdata, std::ptr::null(), -1) };
    assert!(receiver.recv().unwrap().is_err());
}

#[test]
fn callback_can_finish_after_receiver_is_dropped() {
    let (userdata, receiver) = context();
    drop(receiver);
    let filelist = [std::ptr::null()];
    unsafe { open_file_dialog_callback(userdata, filelist.as_ptr(), -1) };
}
