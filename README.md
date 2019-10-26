# 🌳 envsub

Environment variable substitution with [Twig]-inspired transformers.

## Getting started

### Installation

### Usage

```bash
envsub
```

```yaml
containers:
- name: example-container
  image: ${IMAGE|lowercase}
  ports:
  - containerPort: ${PORT|default(80)}
```

## License

© 2019 [Daniel Morris](https://unfun.co)  
Made available under the terms of the [Apache License 2.0](LICENSE.md).

[Twig]: https://twig.symfony.com
[gettext]: https://www.gnu.org/software/gettext/
