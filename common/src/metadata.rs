
/// Declaration for metadata enum, contains data for Token, File, Modules
pub enum Metadata {
    /// Holds metadata for a Token
    Token { 

        /// Line number
        line_num: u16, 
        /// Column number
        col_num: u16 },

    /// Holds metadata for a Module
    Module {
        /// File extension
        file_extension: String,
        /// File size (in bits)
        file_size: u32},

    /// Holds metadata for a Directory
    Directory { 
        /// Path of the directory
        path: String },
}