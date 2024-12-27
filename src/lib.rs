pub mod book;
pub use book::_02_guessing_game;
pub mod kata;
pub mod learnings;

// exporting a public API to hide module details
// so it can be used as
//     use learning_rust::add_one;
// instead of
//     use learning_rust::book::_14_doctest::add_one;
pub use book::_14_doctest::add_one;
