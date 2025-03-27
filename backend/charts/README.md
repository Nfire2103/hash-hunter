# Generate kubeseal secret

Here is an example of how to generate a kubeseal secret:

```bash
echo -n "<SECRET>" | kubeseal --raw --from-file=/dev/stdin --scope cluster-wide --cert charts/kubeseal.pem
```
