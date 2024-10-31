## ISO 20022 API Gateway
This is an API gateway for integrating ISO 20022 messaging standards between internal systems and external networks. 
It will act as a centralized entry point that facilitates seamless communication and data exchange, ensuring compliance with ISO 20022 specifications. 
By managing and transforming requests, it enhances operational efficiency and improves the accuracy of financial transactions across diverse platforms.

### Features

- [x] Transform ISO 20022 XML messages to JSON.
- [x] Validate XML structure against ISO 20022 standards.
- [x] Support for JSON to XML transformations.
- [ ] Support for communication over gRPC

## Usage
Make sure you have Rust and Cargo installed(if not install through [rustup](https://github.com/rust-lang/rustup)). Then, run the following commands:

```bash
git clone https://github.com/iamshabell/osgate
cd osgate
cargo run
```
The server will be available at `http://127.0.0.1:8080`.

##### API Endpoints
- `/transform`: Transforms ISO 20022 XML messages to JSON.
- `/transform/json`: Transforms JSON messages to ISO 20022 XML.
- `/validate`: Validates XML structure against ISO 20022 standards.
