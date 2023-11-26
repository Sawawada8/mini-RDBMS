mod disk_manager;

use std::{
    fs::{File, OpenOptions},
    io::{self, Read, SeekFrom, Write},
};

fn main() -> io::Result<()> {
    let mut dm = disk_manager::DiskManager::open("foo.txt")?;

    let page_id = dm.allocate_page();
    let mut data = Vec::with_capacity(disk_manager::PAGE_SIZE);
    data.extend_from_slice(b"world");
    data.resize(disk_manager::PAGE_SIZE, 0);
    dm.write_page_data(page_id, &data)?;

    let page_id = dm.allocate_page();
    let mut data = Vec::with_capacity(disk_manager::PAGE_SIZE);
    data.extend_from_slice(b"hello");
    data.resize(disk_manager::PAGE_SIZE, 0);
    dm.write_page_data(page_id, &data)?;

    Ok(())
}
