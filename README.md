# envsub

Environment variable substitution.

## Getting started

### Installation

TODO

### Usage

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: nginx
spec:
  containers:
  - image: nginx:${NGINX_VERSION}
    name: nginx
    restartPolicy: Never
```

```bash
NGINX_VERSION=1.23.4 envsub < manifest.yaml
```

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: nginx
spec:
  containers:
  - image: nginx:1.23.4
    name: nginx
    restartPolicy: Never
```

## License

© 2019 [Daniel Morris](https://unfun.co)  
Made available under the terms of the [Apache License 2.0](LICENSE.md).
