<div align=center>

# SUSTech CS323 Project: Incredibuild

C-like Compiler in Rust

</div>

## Build up the project

Setting up environment might be a stuggling process, you can use `docker` to build up the environment.

```bash
docker build -t incredibuild .
```

If you face network issue, you can use two methods to fix it.
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

You can also choose to download the image we have already built from Docker Hub.

```bash
docker pull jaredanjerry/incredibuild:20241106 # Approximately 4.5GB
```

## Set up the project

After building the project, you can set up the project by running an iteractive shell in the container. (Please mount the project root path to `/incredibuild`)

```bash
cd <project-root-path>
docker run -it --rm -v $(pwd):/incredibuild incredibuild
cargo run
```
