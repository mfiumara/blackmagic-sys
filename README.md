# blackmagic-remote

This crate provides Rust bindings for the blackmagic remote protocol. It is mainly built for integration into probe-rs.

## Compiling blackmagic

The `make` process is not yet integrated into the `build.rs` script. For now, first compile blackmagic separately before testing the bindings using the following make command

```
cd blackmagic
ENABLE_RTT=1 PROBE_HOST=hosted HOSTED_BMP_ONLY=1 PC_HOSTED=1 ENABLE_DEBUG=1 make -j 8
```

This builds the BMP app.

## Testing

Some HIL tests have been written to test if everything works with a target connected and debug probe connected. To test on a different system, change the serial number and target voltage in the tests in `lib.rs`.