pub const PAGE_SIZE: u64 = 4096;
pub const TABLE_MAX_PAGES: u64 = 100;
pub const ROW_SIZE: u64 = 307;
pub const ROWS_PER_PAGE: u64 = PAGE_SIZE / ROW_SIZE; // Evaluates to 13 rows
pub const TABLE_MAX_ROWS: u64 = ROWS_PER_PAGE * TABLE_MAX_PAGES;