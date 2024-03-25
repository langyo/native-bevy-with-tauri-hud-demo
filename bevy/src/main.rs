use std::{
    process::{Child, Stdio},
    thread::JoinHandle,
};

use flume::{Receiver, Sender};
use once_cell::sync::Lazy;

use bevy::{prelude::*, winit::WinitWindows};
use raw_window_handle::HasWindowHandle as _;

static EXIT_HANDLER: Lazy<(Sender<JoinHandle<Child>>, Receiver<JoinHandle<Child>>)> =
    Lazy::new(|| {
        let (tx, rx) = flume::unbounded();
        (tx, rx)
    });

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();

    EXIT_HANDLER
        .1
        .recv()
        .unwrap()
        .join()
        .unwrap()
        .kill()
        .unwrap();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    winit_windows: NonSend<WinitWindows>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let window = winit_windows.windows.iter().next().unwrap().1;
    let handle = window.window_handle().unwrap().as_raw();
    match handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            let hwnd: isize = handle.hwnd.into();
            println!("HWND: 0x{:08x}", hwnd);

            let handle = std::thread::spawn(move || {
                std::process::Command::new("./target/debug/_tauri.exe")
                    .arg(hwnd.to_string())
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("failed to execute process")
            });
            EXIT_HANDLER.0.send(handle).unwrap();
        }
        _ => {
            println!("Not a Win32 window");
        }
    }
}
