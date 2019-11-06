# cert-checker
Simple tool for certifcate expiration date validation. It can create alerts for expierd or soon expiring certyficats.

## how to use
The best option would be to use ready docker image, You can find it here.

### docker

### kubernetes

### bare usage

```
cert-checker 1.0
Wojciech Wozniak <wwozniak92@gmail.com>
certyficate checking tool

USAGE:
    cert-checker [OPTIONS] -d <domain>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d <domain>        Domain names to check
    -t <time>          The sleep time between tests [default: 300]
```
By default application will check all domains added after `-d` every 300 seconds.

ex:
```
cert-checker -d www.google.com expired.badssl.com
```

using with opsgenie:




## TODO

- [ ] Log levels for logging
- [ ] Exporting data to prometheus
- [ ] webhook support
- [ ] Add unit tests etc.