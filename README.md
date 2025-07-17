# rtkit-rs

This crate provides a Rust API to request high or realtime priority scheduling
for a process' thread, using [rtkit](https://github.com/heftig/rtkit). The
`rtkit` daemon includes a number of mitigations to help avoid real-time
applications from running away with CPU resources.

Currently, this crate only includes a synchronous API that uses the
`org.freedesktop.RealtimeKit1` D-Bus interface to make calls to the `rtkit`
daemon. In the future, an asynchronous API could also be provided if required
(i.e. please file an issue if you want this).
