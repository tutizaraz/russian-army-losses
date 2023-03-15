Shoporusni CLI
A CLI tool for displaying the latest statistics of the Russian military losses.

This Rust application retrieves the latest statistics of Russian military losses from the russianwarship.rip API and displays them in a formatted table. The application is built using the chrono, colored, prettytable, serde, url, and clap Rust libraries.

Usage
To use the Shoporusni CLI, run the following command in your terminal:

Copy code
shoporusni -s
This will display the latest statistics of Russian military losses in a formatted table. The -s flag is used to indicate that the Shoporusni CLI should be run.

Dependencies
The following Rust libraries are used in this application:

`chrono`: Used for working with dates and times.
`colored`: Used for adding color to terminal output.
`prettytable`: Used for formatting tables in the terminal.
`serde`: Used for serializing and deserializing data in JSON format.
`url`: Used for parsing URLs.
`clap`: Used for parsing command-line arguments.

Author
This Rust application was created by Dmytro Barabash dbarabashdev@gmail.com.
