# RADAR-GUI

![build](https://img.shields.io/badge/build-passing-brightgreen) ![rust](https://img.shields.io/badge/rust-stable-orange) ![license](https://img.shields.io/badge/license-MIT-blue)

## Introduction:
Firstly I designed this item as an data module for Robomaster Radar Robot ,provide server for radar robot's sdr,laser guidance and location camera data modules,by get data from these modules and show them in GUI for user to monitor the radar robot's status and make decision for control the robot
you can understand as follow struct of this item:
+ RADAR-SDR<=> RADAR-GUI
+ RADAR-LISER-GUIDANCE <=> RADAR-GUI
+ RADAR-LOCATION-CAMERA <=> RADAR-GUI 

which related to these repositories
+ [alliance_radar_vision_location](https://github.com/HarryPotter1tech/alliance_radar_vision_location.git)
+ [alliance_radar_sdr](https://github.com/HarryPotter1tech/alliance_radar_sdr.git)
+ [rmcs-laser-guidance](https://github.com/Yukikaze2233/rmcs-laser-guidance.git)
## Start Your First Rust Iced App/Lib:
this is a lib/app build by rust with iced if you want build your first rust iced app/lib ,then run as follows:
```shell 
cargo new radar_gui #this is my item/lib/app name ,you can make by yourself
cd  radar_gui
cargo add iced  #this help you add iced dependence in item's Cargo.toml,auto get the lastest version
```
if you finish you can see follow context like this in Cargo.toml
```toml
[package]
name = "radar_gui"
version = "0.1.0"
edition = "2024"G

[dependencies]
iced = "0.14.0"
```
 now you can start your dev

## Item File Struct:
