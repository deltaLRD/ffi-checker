use std::path::Path;

pub struct AnalysisOption {
    pub crate_names: Vec<String>,
    pub entry_points: Vec<String>,
    pub ffi_functions: Vec<String>,
    pub bitcode_paths: Vec<String>,
    pub precision_threshold: u8, // 0 1 2
}

impl Default for AnalysisOption {
    fn default() -> Self {
        let mut crate_names = vec![];
        let mut entry_points = vec![];
        let mut ffi_functions = vec![];
        let mut bitcode_paths = vec![];
        let mut precision_threshold = 0;

        let entry_points_path = Path::new("target/entry_points");

        let dir = std::fs::read_dir(entry_points_path).unwrap();
        for entry in dir {
            let entry = entry.unwrap();
            let path = entry.path();
            crate_names.push(entry.file_name().into_string().unwrap());
            
        }

        Self {
            crate_names,
            entry_points,
            ffi_functions,
            bitcode_paths,
            precision_threshold,
        }
    }
}
