# phonetrack2prometheus
A flask based server that will exposes a route (with authentication) for a user to send their live location from the [nextcloud phonetrack app](https://github.com/julien-nc/phonetrack/tree/main) to [prometheus](https://github.com/prometheus/prometheus)
- send data to /api
- receive prometheus friendly on /metrics

## get started
1. the .env should look sth like this:
```
FLASK_USERNAME=USERNAME
FLASK_PASSWORD=YOURWELLSECUREDPW
FLASK_HOST=0.0.0.0
FLASK_PORT=5000
```
2. make sure you configure prometheus accordingly:
```
- job_name: 'phonetrack'
    scheme: https
    static_configs:
      - targets: ['address.to.server.that.hosts.phonetrack2prometheus']
    basic_auth:
      username: 'USERNAME'
      password: 'YOURWELLSECUREDPW'
```
4. configure phontrack on your phone with authentication
5. Use Grafana to show metrics from prometheus (adding dashboard json someday)

## Notes
- There are a few things that are not best practice, make sure your know what you are doing. This is just a boilerplate for people who are too lazy to reverse engineer the readings of the phonetrack json
- With this setup, we only handle local network incoming requests obviously.
