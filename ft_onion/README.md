# Ft_Onion

The goal of this project is to create a web page and make it accessible from the Tor net-
work by creating a hidden service. The hidden service is a web service that hides
on the Tor network.

## Technicalities

-> A container (I wanted to do a VM but my USB-key doesn't work lol)
-> Using arch in the container, because I want to try something lighter
-> Using nginx to serve the webpage


## Usage
Docker must be install.

Built and run the container
```bash
make
```
Underlying command:
docker build

See the containers logs
```bash
make log
```

Take down the webserver
```bash
make down
```

Destroy the container and its image
```bash
make clean
```

Rebuild the container from scratch
```bash
make re
```

The container should be accessible on XXXXXXX.onion[xxxxx.onion]
