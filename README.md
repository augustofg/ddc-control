# ddc-control

`ddc-control` is a simple utility to get / set display's VCP parameters via DDC (Data Display Channel). It runs on GNU/Linux and Windows. Big thanks to @arcnmx for providing the necessary rust libraries that made it possible.

Usage:

```bash
$ ./ddc-control -m <monitor_number> get-vcp <vcp_feature_number>
$ ./ddc-control -m <monitor_number> set-vcp <vcp_feature_number> <value>
```

Example:

```bash
$ ./ddc-control -m 1 set-vcp 0x60 0x0F # Set monitor input to DisplayPort
```
