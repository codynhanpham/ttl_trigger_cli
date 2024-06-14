# Send a Single TTL Pulse

This app simply trigger a connected COM device to send a single TTL pulse. The pulse width is determined by the COM baudrate.

## Installation
Download the precompiled binary from the [release page](https://github.com/codynhanpham/ttl_trigger_cli/releases) and that's it.

## Usage
> If running the app without any argument or with `-h` or `--help`, the help message will be displayed.

1. Connect the COM device to the computer. You can get a [cheap USB to TTL converter](https://a.co/d/geoSdmH) from Amazon or eBay for around $12. If you want a BNC output, simply wire the TXD pin to the BNC positive and the GND pin to the BNC negative.
2. Find the COM port of the device. You can do this by running the app with `-l` or `--list` argument:
```bash
./ttl_trigger_cli -l
```
3. Run the app with the following command to send out a single TTL pulse:
```bash
./ttl_trigger_cli -d <COM_PORT> -b <BAUDRATE>
```
where `<COM_PORT>` is the COM port of the device and `<BAUDRATE>` is the baudrate of the device. For example, if the COM port is `COM3` and the baudrate is `9600`, the command would be:
```bash
./ttl_trigger_cli -d COM3 -b 9600
```
*The default baudrate is 9600 if not specified.*

## Important Notes
Since the app sends the TTL via UART, the voltage is held normally high and goes low when the pulse is sent.