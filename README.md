<div align=center>

# SUSTech CS323 Project: Incredibuild

C-like Compiler in Rust

</div>

This project is still under development.

## Features

- [ ] SPL Grammar:
    - [ ] Lexer & Parser
    - [ ] Semantic Analyzer
    - [ ] Macro

## Development

Setting up environment might be a stuggling process, you can use `docker` to build up the environment.

```bash
docker build -t incredibuild .
```

You can also choose to download the image we have already built from Docker Hub.

```bash
docker pull jaredanjerry/incredibuild:20241106 # Approximately 4.5GB
```

After building the project, you can set up the project by running an iteractive shell in the container. (Please mount the project root path to `/incredibuild`)

```bash
cd <project-root-path>
docker run -it --rm -v $(pwd):/incredibuild incredibuild
cargo run
```

## Issue
If you face network issue, there'are two methods to fix it.

1. Modify `/etc/systemd/system/docker.service.d/http-proxy.conf` to add proxy.

    ```conf
    [Service]
    Environment="HTTP_PROXY=http://your-proxy:port"
    Environment="HTTPS_PROXY=http://your-proxy:port"
    ```

    Then restart docker service.

    ```bash
    sudo systemctl daemon-reload
    sudo systemctl restart docker
    ```

2. Add Registry Mirror in `/etc/docker/daemon.json`.

    ```json
    {
        "registry-mirrors": ["<your-mirror>"]
    }
    ```

## Reference

- [llvm@17](https://llvm.org/)
- [lalrpop](https://github.com/lalrpop/lalrpop)
- [Logos](https://github.com/maciejhirsz/logos)
