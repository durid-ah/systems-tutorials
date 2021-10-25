pub const PAGE_SIZE: u64 = 4096;
pub const TABLE_MAX_PAGES: usize = 100;
pub const ROW_SIZE: usize = 307;
pub const ROWS_PER_PAGE: usize = (PAGE_SIZE as usize) / ROW_SIZE; // Evaluates to 13 rows
pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;
