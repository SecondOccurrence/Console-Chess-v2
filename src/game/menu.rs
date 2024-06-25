pub trait MenuFunctions {
    fn show_menu(&self);
    fn perform_command(&self, option: &str);
}
