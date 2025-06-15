# Contributing
Welcome to contributing to this project. For now, this is a rough outline for Issues and Pull Requests.

## Issues
- For bug reports, try to explain the steps to reproduce the following issue.
- The following suggestions, for enhancements, are allowed:
	- Increasing performance.
	- Making certain blocks of code more safe.
	- Improving documentation.
	- Renaming types.

## Pull Requests
All pull requests must follow the listed requirements:
- I would advise against using AI for generating code/documentation.
- For documentation requests, make sure that it successfully compiles with `cargo doc`.
- Code requests, on the other hand, must have each test passed with the following conditions:
	- Including feature `no_std`.
	- Release profile.
	- Debug profile.