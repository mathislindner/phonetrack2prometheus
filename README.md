# phonetrack2prometheus
A flask based server that will exposes a route (with authentication) for a user to send their live location from the [nextcloud phonetrack app](https://github.com/julien-nc/phonetrack/tree/main) that is maintaned by the kind @julien-nc 

## Notes
- Will only work when the phone is online, does not store the time when stat was sent from phone
- With this setup, we only handle local network incoming requests obvs.