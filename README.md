# Measuring the width of unicode graphemes on terminal outputs

Testing the behavior of different unicode graphemes in terminals and checking if the assumed length is fitting to the printed representation.
Uses the `unicode_width` crate for the width estimates. Also check for potential interactions with ANSI coloring.

## Automated measurements

To make it feasible to check many terminal emulators and platforms
automate the counting of columns. Assumes the examples are fitting well
into a row of a standard terminal (e.g. 80x24).
Supports both check without ansi additions (default) and with ansi
background and foreground color changed (`--full`).
Manual checking is possible via `--manual`
