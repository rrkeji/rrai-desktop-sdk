



## examples 

### relay

```
cargo run --example relay -- --secret-key-seed=42 --port 6080
```

### dcurt

```
RUST_BACKTRACE=full cargo run --example dcurt -- --mode listen --secret-key-seed 42 --relay-address /ip4/127.0.0.1/tcp/6080/p2p/12D3KooWR2KSRQWyanR1dPvnZkXt296xgf3FFn8135szya3zYYwY
```

```
RUST_BACKTRACE=full cargo run --example dcurt -- --mode listen --secret-key-seed 42 --relay-address /ip4/49.232.102.140/tcp/6080/p2p/12D3KooWR2KSRQWyanR1dPvnZkXt296xgf3FFn8135szya3zYYwY
```

```
RUST_BACKTRACE=full cargo run --example dcurt -- --mode dial --secret-key-seed 42 --relay-address /ip4/127.0.0.1/tcp/6080/p2p/12D3KooWR2KSRQWyanR1dPvnZkXt296xgf3FFn8135szya3zYYwY --remote-peer-id 12D3KooWCM71YCZnUg2TfsV5wwmsU4dB6f9bVSsX4o4x3UsgtkjD
```

### ping

```
cargo run --example ping -- /ip4/127.0.0.1/tcp/6080
```