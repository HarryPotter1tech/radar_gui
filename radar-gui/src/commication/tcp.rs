use std::io::{Error,Read, Write};
use std::net::{TcpListener,TcpStream};
use std::thread;
use std::time;
use chrono;
const RADAR_SDR_PORT: &str = "127.0.0.1:4000";
const RADAR_LASER_GUIDANCE_PORT: &str = "127.0.0.1:6000";
const RADAR_LOCATION_CAMERA_PORT: &str = "127.0.0.1:8000";

const RADAR_SDR_NAME: &str = "RADAR_SDR";
const RADAR_LASER_GUIDANCE_NAME: &str = "RADAR_LASER_GUIDANCE";
const RADAR_LOCATION_CAMERA_NAME: &str = "RADAR_LOCATION_CAMERA";
fn tcp_server_start(){
    println!("TCP server is starting...");
    let listen_sdr =TcpListener::bind(RADAR_SDR_PORT).expect("Failed to bind TCP server for RADAR_SDR");
    println!("TCP server[{}] is listening on {}", RADAR_SDR_NAME, RADAR_SDR_PORT);
    let listen_laser_guidance=TcpListener::bind(RADAR_LASER_GUIDANCE_PORT).expect("Failed to bind TCP server for RADAR_LASER_GUIDANCE");
    println!("TCP server[{}] is listening on {}", RADAR_LASER_GUIDANCE_NAME, RADAR_LASER_GUIDANCE_PORT);
    let listen_location_camera=TcpListener::bind(RADAR_LOCATION_CAMERA_PORT).expect("Failed to bind TCP server for RADAR_LOCATION_CAMERA");
    println!("TCP server[{}] is listening on {}", RADAR_LOCATION_CAMERA_NAME, RADAR_LOCATION_CAMERA_PORT);
    (listen_sdr, listen_laser_guidance, listen_location_camera)
    
}
fn tcp_server_run(listen_sdr: TcpListener, listen_laser_guidance: TcpListener, listen_location_camera: TcpListener){
    let thread_sdr = thread::spawn(move || {
        tcp_server_connect(listen_sdr, RADAR_SDR_NAME);
    });;
    let thread_laser_guidance = thread::spawn(move || {
        tcp_server_connect(listen_laser_guidance, RADAR_LASER_GUIDANCE_NAME);
    });
    let thread_location_camera = thread::spawn(move || {
        tcp_server_connect(listen_location_camera, RADAR_LOCATION_CAMERA_NAME);
    });
}
fn tcp_server_stop(thread_sdr: thread::JoinHandle<()>, thread_laser_guidance: thread::JoinHandle<()>, thread_location_camera: thread::JoinHandle<()>){
    println!("TCP server is stopping...");
    thread_sdr.join().expect("Failed to stop TCP server for RADAR_SDR");
    thread_laser_guidance.join().expect("Failed to stop TCP server for RADAR_LASER_GUIDANCE");
    thread_location_camera.join().expect("Failed to stop TCP server for RADAR_LOCATION_CAMERA");
    println!("TCP server stopped.");
}

fn tcp_server_connect( tcp_listener: TcpListener, server_name: &str){
    loop {
        let(stream, addr) = tcp_listener.accept().unwrap();
        println!("[{}] client connected: {}", server_name, addr);
        tcp_server_message_get(stream, server_name);
    }
}
fn tcp_server_message_get(mut stream: TcpStream, server_name: &str){
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("[{}] client disconnected: {}", server_name, stream.peer_addr().unwrap());
                break;
            }
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("[{}] received message from {}: {}", server_name, stream.peer_addr().unwrap(), message);
            }
            Err(e) => {
                println!("[{}] error reading from {}: {}", server_name, stream.peer_addr().unwrap(), e);
                break;
            }
        }
    }
}
