# 🌳 envsub

Environment variable substitution with Bash-esque variable expansion.

## Getting started

### Installation and usage

```bash
envsub
```

```yaml
containers:
- name: example-container
  image: ${IMAGE}
  ports:
  - containerPort: ${PORT}
```

## License

© 2019 [Daniel Morris](https://unfun.co)  
Made available under the terms of the [Apache License 2.0](LICENSE.md).
