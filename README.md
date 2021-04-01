# Fund-watch-bot

Fund-watch-bot is telegram bot for subscribing on submitting SEC form 13F.

## Build
```bash
docker build -t fund-watch-bot -f ./Dockerfile .
```

## Import
```bash
docker run -it fund-watch-bot /usr/src/app/fund-watch-bot import-13f-form
```

## Run
```bash
docker run fund-watch-bot 
```