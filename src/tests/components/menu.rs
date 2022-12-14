use std::path::Path;

use crate::components::menu::Menu;
use crate::models::dialogable::MockDialogable;
use crate::models::menu::Menu as MenuModel;
use crate::models::record::Recording;

fn setup() -> Menu {
    Menu::new()
}


#[tokio::test]
async fn force_volume() {
    let mut menu = setup();
    let value = 0.0;
    let volume = menu.set_volume(value);
    assert!(volume == value);
    let volume = menu.set_volume(value);
    assert!(volume == value);
}

/*
    search & save scenarios # 2
    successfull # 2
    fail_completely # 4
    partially_fails # 6
*/

#[tokio::test]
async fn search_midi() {
    let mut menu = setup();
    let diag = MockDialogable::new();
    let mut bdiag = Box::new(diag);
    MockDialogable::expect_open(bdiag.as_mut()).times(1);
    MockDialogable::expect_close(bdiag.as_mut())
        .return_const(Ok("existent_file".into()))
        .times(1);
    let path = menu.search_midi(bdiag).await;
    assert!(path.is_ok());
    let resultpath = path.unwrap();
    assert!(Path::new(&resultpath).exists());
}

#[tokio::test]
async fn search_unexistent_midi() {
    let mut menu = setup();
    let diag = MockDialogable::new();
    let mut bdiag = Box::new(diag);
    let err = "unexistent midi".into();
    MockDialogable::expect_open(bdiag.as_mut()).times(1);
    MockDialogable::expect_close(bdiag.as_mut())
        .return_const(Err(err))
        .times(1);
    let path = menu.search_midi(bdiag).await;
    assert!(path.is_err());
    let resultpath = path.unwrap();
    assert!(!Path::new(&resultpath).exists());
}

#[tokio::test]
async fn save_record() {
    let mut menu = setup();
    let diag = MockDialogable::new();
    let mut bdiag = Box::new(diag);
    let mut recording = Recording::new();
    recording.push(String::from("sample record"));
    MockDialogable::expect_open(bdiag.as_mut()).times(1);
    MockDialogable::expect_close(bdiag.as_mut())
        .return_const(Ok("existing_save_record".into()))
        .times(1);
    let path = menu.save_recording(bdiag, recording).await;
    assert!(path.is_ok());
    let resultpath = path.unwrap();
    assert!(Path::new(&resultpath).exists());
}

#[tokio::test]
async fn save_empty_record() {
    let mut menu = setup();
    let diag = MockDialogable::new();
    let mut bdiag = Box::new(diag);
    let recording = Recording::new();
    let err = "empty recording".into();
    //recording.push(String::from("sample record")); // Don't push anything so it fails;
    MockDialogable::expect_open(bdiag.as_mut()).times(1);
    MockDialogable::expect_close(bdiag.as_mut())
        .return_const(Err(err))
        .times(1);
    let path = menu.save_recording(bdiag, recording).await;
    assert!(path.is_err());
    let resultpath = path.unwrap();
    assert!(!Path::new(&resultpath).exists());
}
