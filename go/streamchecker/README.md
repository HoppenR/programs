# Streamchecker

Currently all credentials are supplied at compile time with `-ldflags`:

Example:

`go build -ldflags "-X main.clientID=XXX -X main.tmptoken=XXX -X main.userID=XXX " streamchecker`

Explanation:

`clientID`: The api key of your dev.twitch.tv project

`tmptoken`: Temporarily used hardcoded app access token

`userID`: the ID of your twitch account, to get one you will have to use the
legacy api for now, because the new one requires user permissions, example request:

```sh
curl -H 'Accept: application/vnd.twitchtv.v5+json' \
-H 'Client-ID: XXX' \
-X GET https://api.twitch.tv/kraken/users?login=XXX
```

# Navigation
standard vim navigation: `jkl` or arrow keys + enter
`q` to quit
