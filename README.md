# Daedalus Rex

<div align="center">
  
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Version](https://img.shields.io/badge/version-1.0.0-green.svg)
![Status](https://img.shields.io/badge/status-prototype-orange.svg)

*Advanced Fixed-Wing Unmanned Aerial Platform with Mesh Networking Capabilities*

</div>

## Overview

Daedalus Rex is a sophisticated fixed-wing unmanned aerial platform designed for extended range operations and advanced communication capabilities. The system integrates cutting-edge mesh networking technology with modern aerodynamic design principles.

## Key Features

- **Extended Range Operations**: 6-foot wingspan with optimized Naca 420 airfoils
- **Advanced Powertrain**: Dual Electric Ducted Fan configuration with BLDS 80 MPSC controllers
- **Mesh Network Integration**: Distributed communication architecture for reliable data transmission
- **Modular Design**: Carbon fiber rigid shell exoskeleton with pultruded carbon fiber reinforcement
- **V-Tail Configuration**: Downward V-tail design for improved control characteristics
- **Autonomous Navigation**: ESP32-based flight control system with GPS integration

## Technical Specifications

### Airframe
- **Wingspan**: 6 feet
- **Airfoil**: Naca 420
- **Construction**: Carbon fiber composite
- **Tail Configuration**: Downward V-tail
- **Control Surfaces**: Servo-actuated

### Electronics
- **Flight Controller**: ESP W32 Rover
- **Telemetry**: SIM 7000G
- **Power System**: 2x SYPOM 6000 LiPo Batteries
- **Motors**: Dual BLDS 80 MPSC Electric Ducted Fans
- **Communication**: GPU antenna for telemetry

### Network Architecture
- **Protocol**: Custom mesh networking implementation
- **Language**: Rust-based networking stack
- **Features**: 
  - Distributed node communication
  - Interference resistance
  - Autonomous network formation

## Build Requirements

### Tools
- 3D Printer (for tail and structural components)
- Precision caliper
- Grinding tool (for carbon fiber rod modification)
- Basic assembly tools

### Materials
- Pultruded carbon fiber rods
- Epoxy adhesive
- 3D printing filament
- Electronic components as per BOM

## Assembly Guide

1. **Preparation**
   - Print required STL files (tail section, shoulder components)
   - Cut carbon fiber rods to specified lengths
   - Prepare electronic components

2. **Main Structure**
   - Assemble carbon fiber frame
   - Mount V-tail components
   - Install wing spars

3. **Electronics Integration**
   - Mount EDF units
   - Install flight controller and power system
   - Connect control servos
   - Set up telemetry system

## Software Architecture

```
daedalus/
├── firmware/
│   ├── flight_control/
│   └── motor_control/
├── mesh_network/
│   ├── node_communication/
│   └── telemetry/
└── ground_control/
```

## Team

- Jeremy Nixon - Project Lead
- Evan Rusmisel - Manufacturing
- Dave - Systems Integration
- Balaji Akkayyagari - Frame Construction
- Ernest Yeung - Electronics
- Will Diamond - Network Architecture
- Aditya Advani - Testing

## Development Status

This project is currently in prototype phase. All designs and specifications are subject to change based on testing results and operational requirements.

## Contributing

Please read `CONTRIBUTING.md` for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the `LICENSE.md` file for details.

## Acknowledgments

Special thanks to all team members and contributors who have made this project possible.

---

*Note: This README is a living document and will be updated as the project evolves.*

