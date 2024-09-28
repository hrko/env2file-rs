# env2file-rs

A Rust-based utility to create configuration files within containers by populating them with content passed via environment variables. This is particularly useful in scenarios where you need to dynamically generate config files at container startup based on environment-specific settings.

## Download

You can download the latest release from the [releases page](https://github.com/hrko/env2file-rs/releases/latest).

## Usage

`env2file-rs` is intended to be used as an entrypoint for a container. 

### Example Dockerfile

Below is an example of how to use `env2file-rs` in a Dockerfile:

```Dockerfile
FROM debian:bookworm-slim AS env2file-downloader
RUN apt-get update && apt-get install -y curl
RUN if [ "$(uname -m)" = "x86_64" ]; then ARCH="amd64"; else ARCH="arm64"; fi && \
    curl -L -o /usr/local/bin/env2file https://github.com/hrko/env2file-rs/releases/download/v1.0.0/env2file-rs-$ARCH && \
    chmod +x /usr/local/bin/env2file

FROM <original-image>
COPY --from=env2file-downloader /usr/local/bin/env2file /usr/local/bin/env2file
ENTRYPOINT ["/usr/local/bin/env2file", "<original-entrypoint...>"]
```

### Environment Variables

At container startup, set environment variables following the specified format:

* Content: `ENV2FILE_<ID>_CONTENT`: The actual content to be written to the file.
* Metadata: `ENV2FILE_<ID>_META`: A JSON string containing file metadata:
  * `path`: The path where the file should be created (required).
  * `owner`: (Optional) The desired owner of the file (username or UID).
  * `group`: (Optional) The desired group of the file (group name or GID).
  * `mode`: (Optional) The desired file permissions (octal format, e.g., "644").

Example:

```bash
ENV2FILE_CONFIG_CONTENT="$(cat /path/to/config.json)"
ENV2FILE_CONFIG_META='{"path":"/etc/config.json","owner":"appuser","group":"appuser","mode":"644"}'
```

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests.
