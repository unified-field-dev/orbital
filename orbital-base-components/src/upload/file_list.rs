//! Browser file selection payload for upload handlers.

pub use web_sys::FileList as UploadFileList;

#[cfg(test)]
mod tests {
    use super::UploadFileList;

    #[test]
    fn upload_file_list_is_web_sys_alias() {
        fn assert_file_list(_: UploadFileList) {}
        let _ = assert_file_list;
    }
}
