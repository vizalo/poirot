<p align="center">
    <img src="poirot-logo.png" width="200" height="200"/>
</p>

# Poirot

The detective that watches system resource usage.

Poirot can currently monitor:
* CPU usage
* Memory usage
* Filesystem usage
* CPU temp
* Uptime

### Installation

> [!IMPORTANT]
> To start using Poirot you will need to make sure you have the Poirot app up and running, you can find setup instructions [here](https://github.com/vizalo/poirot-app/blob/main/README.md#installation)

On your server you can install Poirot with:
```sh
curl https://poirot.vizalo.app/install.sh | sh
```

###### What it does?

The install script does a few things:
1. It will ask you to input your Poirot app URL and your API token
2. It downloads and places the Poirot executable in `$HOME/bin`
3. It sets up a cron job to run every minute to execute Poirot