# cert-checker
Simple tool for certifcate expiration date validation. It can create alerts for expierd or soon expiring certyficats.

## how to use
The best option would be to use ready docker image, You can find it here.

### docker
to run in docker just start
```
docker run wojtekw92/cert-checker -d www.google.com expired.badssl.com
```

### kubernetes

in `k8s` dir you have ready to use deployment file you just need to chage domains and apply it to your cluster

### bare usage

```
cert-checker 1.0
Wojciech Wozniak <wwozniak92@gmail.com>
Certificate checking tool

USAGE:
    cert-checker [FLAGS] [OPTIONS] -d <domain>

FLAGS:
    -h, --help       Prints help information
    -j               json output log
    -V, --version    Prints version information

OPTIONS:
    -d <domain>         Domain names to check
    -l <left>           Time before expiration that should be warning in days [default: 5]
    -o <API-KEY>        opsgenie intergration to infrom about soon failing certs
    -t <time>           The sleep time between tests [default: 300]
    -w <URL>            Webhook for failed or failing soon certs
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
- [x] Opsgenie Integration
