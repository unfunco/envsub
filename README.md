# envsub

**WIP**

CLI tool designed to simplify the process of placeholder replacement in
text files. It's similar to [envsubst] in that it is primarily used for
injecting environment variables into configuration files.

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

### Filters

|           Filter | Description                                                      |
|-----------------:|------------------------------------------------------------------|
| `default(value)` | Replaces the input with the default value if the input is empty. |
|      `lowercase` | Transforms the characters in the input to lower case.            |
|      `uppercase` | Transforms the characters in the input to upper case.            |

## License

© 2019 [Daniel Morris](https://unfun.co)  
Made available under the terms of the [Apache License 2.0](LICENSE.md).

[envsubst]: https://www.gnu.org/software/gettext/manual/html_node/envsubst-Invocation.html
