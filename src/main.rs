use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// the COM device to send the serial request to, case-sensitive
    #[arg(short = 'd', long = "device", default_value = "<None>", conflicts_with = "list_ports", required_unless_present = "list_ports", default_value_if("list_ports", "true", "<None>"))]
    com_device: String,

    /// the baudrate to send the serial request at
    #[arg(short, long, default_value = "9600")]
    baudrate: u32,

    /// list all available COM ports and exit (the value for COM devices are highlighted in yellow)
    #[arg(short = 'l', long = "list", conflicts_with = "com_device", default_value_if("com_device", "None", "true"))]
    list_ports: bool,
}

fn get_com_port_info(port_type: &serialport::SerialPortType) -> String {
    match port_type {
        serialport::SerialPortType::UsbPort(info) => {
            format!(
                "{} - {} ({})",
                info.manufacturer.as_ref().map_or("Unknown", String::as_str),
                info.product.as_ref().map_or("Unknown", String::as_str),
                info.serial_number.as_ref().map_or("Unknown", String::as_str)
            )
        }
        serialport::SerialPortType::PciPort => "PCI Port".to_string(),
        serialport::SerialPortType::BluetoothPort => "Bluetooth Port".to_string(),
        serialport::SerialPortType::Unknown => "Unknown".to_string(),
    }
}

/// List all the available serial ports
fn list_ports() {
    let ports = serialport::available_ports().expect("No serial ports found!");

    if ports.is_empty() {
        println!("\nNo serial ports found!\n");
        return;
    }

    println!("\nThere are {} serial ports available:\n", ports.len());

    for port in ports {
        let product = get_com_port_info(&port.port_type);
        println!("• Info: {}", product);
        println!("\tDevice ID: {}", port.port_name.yellow());
    }
    println!();
}

/// Make sure the selected COM port is available
fn validate_com_port(com_device: &str) -> bool {
    let ports = serialport::available_ports().expect("\nNo serial ports found! Run the program with `--list` to list all available ports.\n");

    if ports.is_empty() {
        println!("\nNo serial ports found! Run the program with {} to list all available ports.\n", "--list".dimmed());
        return false;
    }

    for port in ports {
        if port.port_name == com_device {
            return true;
        }
    }

    println!("\nThe COM device '{}' is not available. Run the program with {} to list all available ports.\n", com_device.yellow(), "--list".dimmed());

    false
}


/// Send a TTL pulse (with 1 byte 0xFF) to the selected COM port
fn main() {
    let args = Args::parse();

    if args.list_ports || args.com_device == "<None>" {
        list_ports();
        return;
    }

    if !validate_com_port(args.com_device.as_str()) {
        return;
    }


    // Initialize the serial port
    let mut serial_port = serialport::new(args.com_device.as_str(), args.baudrate)
        .open()
        .expect("Failed to open serial port");

    // Send the TTL pulse
    serial_port.write_all(&[0xFF]).expect("Failed to write to serial port");
    println!("\nTTL pulse sent to COM device '{}' at the baudrate of {}.\n", args.com_device.yellow(), args.baudrate.to_string().green());

    // Close the serial port
    std::mem::drop(serial_port);
}