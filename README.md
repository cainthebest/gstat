# gstat

gstat is a project that is currently in its early stages and is not yet functional. It is intended to be a tool for gathering data from game servers.

## Contributing

Contributions to gstat are welcome and greatly appreciated. Please see the CONTRIBUTING.md file for guidelines on how to contribute.

## Code of Conduct

Please note that this project adheres to the Contributor Covenant Code of Conduct. By participating in this project, you are expected to uphold the code of conduct.

## Goals

- [ ] Create a standard across the lib for the main aspects of a game server query (e.g. protocol, query, response, etc).
- [ ] Insure that the standard is flexible enough to be used for all types of games (With the exception of games that use a proprietary protocol that cannot be reverse engineered or games that do not use a query/response system).
- [ ] Stick to a modular design that allows for easy addition of new games, protocols, and tools.
- [ ] Support FFI for use in other languages (e.g. Dynamic libraries for C/C++, bindings for other languages, etc).
- [ ] Create a CLI tool that can be used to query game servers and display the results.
- [ ] Have our own documentation for game protocols and queries, so that we do not have to rely on other sources that may not be accurate or up to date.

## License

gstat is licensed under the GNU General Public License v3.0. Please see the LICENSE file for more information.
