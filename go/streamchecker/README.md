# Streamchecker

Currently all credentials are supplied at compile time with `-ldflags`:

Example:

`go build -ldflags "-X main.clientID=XXX -X main.tmptoken=XXX -X main.userID=XXX " streamchecker`

Explanation:

`clientID`: The api key of your dev.twitch.tv project

`tmptoken`: Temporarily used hardcoded app access token, to get an app access
token you can use [this](https://github.com/twitchdev/authentication-go-sample/blob/master/oauth-client-credentials/main.go)

`userID`: the ID of your twitch account, to get one you will have to use the
legacy api for now, because the new one requires user permissions, example request:

```sh
curl -H 'Accept: application/vnd.twitchtv.v5+json' \
-H 'Client-ID: XXX' \
-X GET https://api.twitch.tv/kraken/users?login=XXX
```

# Navigation
standard vim navigation: `jkl` or arrow keys + enter

`f` to open a filter dialog

`F` to clear filter

filters starting with `!` shows only non-matching results

filter window supports regular readline keys such as ctrl-u to clear, ctrl-a to
go to beginning of line, ctrl-e to go to end of line etc

`q` to quit
