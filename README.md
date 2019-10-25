# envsub

## Getting started

```yaml
apiVersion: v1
kind: Deployment
metadata:
  labels:
    app: example
  name: example-deployment
spec:
  replicas: 1
  selector:
    matchLabels:
      app: example
  template:
    metadata:
      labels:
        app: example
    spec:
      containers:
      - name: example
        image: ${IMAGE}
        ports:
        - containerPort: ${PORT}
```

```bash
export IMAGE="nginx:latest"
PORT="80" envsub < deployment.yaml > /tmp/deployment.yaml
cat /tmp/deployment.yaml
```

## License

© 2019 [Daniel Morris](https://unfun.co)  
Made available under the terms of the [Apache License 2.0](LICENSE.md).
