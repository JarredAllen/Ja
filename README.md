# Ja 
A clone of yes from GNU Coreutils, but optionally allowing you to specify a
delay between each time it prints out its argument.

## Installation
#### From source
To build this project from source, download the source via:

```bash
git clone git@github.com:JarredAllen/Ja.git
```

You can then make whatever changes you want to the source, and then build by
running:

```bash
cargo build --release
```

#### From binary
The latest binary executable can be found under the releases tab.

## Usage
`ja "y"`: Prints out y repeatedly (identical to default behavior of yes)

`ja --delay 0.5 "y"`: Prints out y, with a half second delay between each print.

## Contributing
If you have any functionality which you would like to add to this project, fork
it, write your functionality, and then submit a pull request. You should also
run rustfmt on your code before submitting it, and document all functionality in
your code.

## Credits
Originally created by Jarred Allen <jarredallen73@gmail.com>.

## Name
The project is named ja, because ja is the german word for yes and because those
are the initials of the original creator of this project.

## License
This project is licensed under the [MIT License](./LICENSE).
