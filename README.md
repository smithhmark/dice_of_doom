# dice_of_doom
Dice of Doom, but in Rust

The book _Land_of_Lisp_ by Conrad Barski has an excellent example game called "Dice of Doom". This is a version of that game written in Rust.

# Organization
currently, the data model and controller are just in the src/ directory.

In the future, there will be a doom module, with sub-modules for the model, controller, and any available views. This will allow main to invoke the appropriate view for player interaction.

