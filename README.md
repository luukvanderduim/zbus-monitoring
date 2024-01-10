# MonitoringProxy defaults on zbus 3.14.1

The default `destination` and `path` set by `MonitoringProxy::new` in zbus 3.14.1 are wrong. These are corrected in current git. (2024-01-10)

## Usage

The crate contains a few small binaries, `monitor_defaults` with the defaults and `monitor_built` manually built with correct path and destination.

The first one will fail using zbus 3.14.1, likely earlier versions too.

```bash
cargo run --bin monitor_defaults
```

```bash
cargo run --bin monitor_built
```

The default destination: "org.freedesktop.DBus.Monitoring" is not a service, but an interface and yields:

Error: ServiceUnknown("The name is not activatable")

The default destination should be: "org.freedesktop.DBus"

When the destination is corrected, eg. using the builder, we find that
the default path: "/org/freedesktop/Monitoring"

Error: AccessDenied("Invalid object path")

The correct path is "/org/freedesktop/DBus"

## Notes

### Eavesdropping

Monitoring seems to work fine without "eavesdropping=yes" argument in the `MatchRule`.

### Stray signal

Even if a `MatchRule` is passed to `MonitoringProxy::become_monitor` to monitor messages of type 'method_call' and member='Notify', this receives a `NameLost` signal before filtering subsequent signals.

Presumably this one was already in queue.

## monitor_notify

```bash
cargo run --bin monitor_notify
```

Will monitor for `Notify` method calls.
