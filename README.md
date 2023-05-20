# envsub

**WIP**

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
    ports:
    - containerPort: ${PORT | default(80)}
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
    ports:
    - containerPort: 80
    restartPolicy: Never
```

## License

© 2019 [Daniel Morris](https://unfun.co)  
Made available under the terms of the [Apache License 2.0](LICENSE.md).
