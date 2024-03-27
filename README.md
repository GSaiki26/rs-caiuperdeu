# Rs CaiuPerdeu
[![Audit](https://github.com/GSaiki26/rs-caiuperdeu/actions/workflows/audit.yaml/badge.svg)](https://github.com/GSaiki26/rs-caiuperdeu/actions/workflows/audit.yaml) [![Docker build](https://github.com/GSaiki26/rs-caiuperdeu/actions/workflows/docker-build.yaml/badge.svg)](https://github.com/GSaiki26/rs-caiuperdeu/actions/workflows/docker-build.yaml) [![Docker Hub](https://github.com/GSaiki26/rs-caiuperdeu/actions/workflows/docker-hub.yaml/badge.svg)](https://github.com/GSaiki26/rs-caiuperdeu/actions/workflows/docker-hub.yaml) [![Linter](https://github.com/GSaiki26/rs-caiuperdeu/actions/workflows/linter.yaml/badge.svg)](https://github.com/GSaiki26/rs-caiuperdeu/actions/workflows/linter.yaml)

The `Rust CaiuPerdeu` is a Rust remaster of the original bot: [CaiuPerdeu](https://github.com/GSaiki26/py-caiuperdeu).
It's a discord bot client that checks all players in a voice chat until 1 is left. A game of resistence.

## Deploy
You can get the caiuperdeu's docker image using docker hub:
```sh
docker run --ti --env-file ./app.env --name caiuperdeu gsaiki26/rs-caiuperdeu;
```

Or, if you want to build by yourself, the `Dockerfile` is available in the root path.

# Environment variables
An example environment file can be found by `app.env.example`.

* `DISCORD_TOKEN`: The bot's token that can be found in: [Discord Developer Portal](https://discord.com/developers/applications).
* `COOLDOWN_TIME_MS`: The cooldown time to check the players in the voice chat.
