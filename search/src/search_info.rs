
pub struct SearchInfo {
    pub nodes_searched: usize,
}

impl Default for SearchInfo {
    fn default() -> Self {
        Self{nodes_searched: 0}
    }
}