use crate::extract::ExtractedData;
pub(crate) trait PrintBlock {
    fn print_block(&self, extracted_ &ExtractedData) -> String;
}
