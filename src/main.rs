use relm4::RelmApp;

pub mod encryption;
pub mod gui;
pub mod model;
pub mod requests;
pub mod tests;

fn main() {
    tests::test_encryption::test_encryption();
}
