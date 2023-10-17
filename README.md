# parsec-example

A minimal configuration for Parsec, with comments explaining the code, and commented code that can be uncommented for extra configuration options.

# Structure

The structure of a normal Parsec configuration workspace consists of two members:

 - A config library, with files in "`./src/`";
 - A binary crate, in "`./application/`";

The config library is where all the configuration should take place. You almost definitely should not mess with the binary crate, whose only real purpose is to allow for hot lib reloading.

Inside of the config crate, a function will be defined that returns a `SessionCfg<U, I>`, where `U` is the chosen `Ui` (.e.g `parsec_term::Ui`), and `I` is the `InputMethod` for `File`s (.e.g `parsec_kak::Editor`). This `SessionCfg` will be used to start a Parsec `Session`, thus running the program.
