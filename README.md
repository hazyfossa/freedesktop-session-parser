# freedesktop-session-parser
A simple parser for linux session entries. It implements only the subset of `defstop-entry` specification relevant for session managers.

This project is in no way an official implementation, nor is it endorsed by freedesktop.org.

# non-goals
The following spec features will not be supported:
* Groups (other than default)
* Recognized keys that are irrelevant to a session definition
* Icon resolution

# todo
- [ ] locale strings support
- [+] environment integration