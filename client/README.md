# Control Client for Reference Value Provider Service

On one terminal, launch the server
```bash
cargo run -- --control-addr 0.0.0.0:8000 --ac-addr 0.0.0.0:8001
```

On the other terminal, launch a client
```bash
cargo run -- --rvps-addr http://127.0.0.1:8000 --path ../tests/message/intoto.json
```

Here, the `rvps_addr` parameter of client side must have a
`http://` prefix.