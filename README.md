# TLS_TASK
## Building
```shell
cargo build
```
## Running
Run UDS Server
```shell
unixserver
```
Run TCP server
```shell
cargo run --bin server -- --certs a --key a uds
```
Run the client
```shell
echo tls sample communication | cargo run --bin client -- localhost
```
** tls command The following content means the contents of the data transmitted from the TLS requester to the TLS server.
## TLS
The server program and client program of the project carry out TLS communication.
In other words, all communication packets between service machines and requesters are encrypted and communicated with the TLS authentication method.
There is a certs directory in the project, which contains ca.cert, end.fullchain, and end.rsa files.
The end.fullchain and end.rsa files are stored in the service machine, and the ca.cert file is stored in the request machine.
The service machine uses the end.fullchain file as a certificate file and the end.rsa file as a key file.
The client machine uses the ca.cert file as a certificate file to access the service machine.
    *The certificate files exist in the project's certs directory, which must be copied to the target/debug directory after building the project.
The certificate and key file paths are implemented to be obtained by referring to the contents described in the config.yaml file.
# COMMUNICATION
While executing the lightning arrester program, communication with the corresponding sink proceeds according to the parameters of echo, http, and uds.
## Console
If the echo parameter is given later while starting the requester program, the servicer displays the data received from the requester on the console screen.
## Unix Data Stream
First, start the unixsocket program.
This program proceeds with data communication by referring to the socket file path described in the config.yaml file.
If you enter the uds parameter later while starting the requester program, the servicer decodes the data sent from the requester and transmits the data to the unixsocket program in UnixDataStream method.
At this time, the unixsocket program is implemented to display the data.
In other words, the data transmitted from the TLS requester is decrypted by the TLS server, and then the communication is carried out to the unixsocket program.
