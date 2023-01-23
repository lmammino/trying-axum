# trying-axum

Learning the [axum](https://github.com/tokio-rs/axum) Rust web framework live on [Twitch](https://twitch.tv/loige).

---

This repository contains the code we developed while live-streaming on Twitch. We tried to implement a simple web server that can receive and store notes in memory.

The recordings are available on [Youtube](https://www.youtube.com/playlist?list=PLbNOKnE-Oyr2M0ixEaZqgmpFhN1fZhbbK).

More to come in the next streams, make sure to follow us if you are interested in learning along.

## How to run

Clone the repo and then execute

```bash
cargo run
```

You should now have the server running locally at [localhost:3000](http://localhost:3000).

## Endpoints

The endpoints developed for now are:

- `POST /note`: create a new note (based on the string passed as body payload)
- `GET /note/:id`: gets a note by ID


## Contributing

Everyone is very welcome to contribute to this project.
You can contribute just by submitting bugs or suggesting improvements by
[opening an issue on GitHub](https://github.com/lmammino/trying-axum/issues).


## License

Licensed under [MIT License](LICENSE). © Luciano Mammino.
