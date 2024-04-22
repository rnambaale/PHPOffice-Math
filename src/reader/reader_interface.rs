pub trait ReaderInterface {
    fn read(&mut self, content: &str) -> Result<(), String>;
}
