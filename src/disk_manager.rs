use std::io::Seek;
use std::{
    fs::{File, OpenOptions},
    io::{self, Read, SeekFrom, Write},
    path::Path,
};

/// 第２章ディスクマネージャーの実装
///
/// 役割
/// rdbms は永続化のためにデータをファイルに書き込む
/// ディスクマネージャーは、ファイルの読み書きを担う
///
/// ヒープファイル
/// ファイルを固定長の長さ（ページ）ごとに区切ったもの
///
/// ページ
/// 大体 4096 バイト
/// HDDやSDDも書き込むときに特定のサイズ（ブロック）で書き込む
/// このときのサイズがこれくらい（linux ext4 では default のブロックサイズが 4096）
/// これくらいに設定すれば無駄なく扱える
///
/// 実装
/// ページIDの採番（ページの新規作成）
/// ヒープファイルからページの書き込み、読み取り

#[derive(Debug)]
pub struct DiskManager {
    heap_file: File,
    next_page_id: u64,
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct PageId(pub u64);
// new type pattern というらしい

pub const PAGE_SIZE: usize = 4096;

impl DiskManager {
    pub fn new(heap_file: File) -> io::Result<Self> {
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;
        Ok(Self {
            heap_file,
            next_page_id,
        })
    }

    pub fn open(heap_file_path: impl AsRef<Path>) -> io::Result<Self> {
        let heap_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(heap_file_path)?;
        Self::new(heap_file)
    }

    pub fn allocate_page(&mut self) -> PageId {
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        PageId(page_id)
    }

    pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {
        // このタイプのストラクとは、taple っぽく値が取得できるみたい
        let offset = PAGE_SIZE as u64 * page_id.0;

        // file の先頭から数えて offset バイト目へ
        self.heap_file.seek(SeekFrom::Start(offset))?;

        self.heap_file.write_all(data)
    }

    pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
        let offset = PAGE_SIZE as u64 * page_id.0;
        self.heap_file.seek(SeekFrom::Start(offset))?;

        // 読みだしたデータをそのまま帰すのではなく、data に書き込む
        self.heap_file.read_exact(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod helper {
        use std::fs;

        use super::*;

        pub fn init(path: &str) -> DiskManager {
            // let heap_file = File::create("foo.txt")?;
            // let dm = DiskManager::new(heap_file)?;
            let dm = DiskManager::open(path).unwrap();
            dm
        }

        pub fn cleanup(path: &str) -> std::io::Result<()> {
            fs::remove_file(path)?;
            Ok(())
        }
    }

    #[test]
    fn new() {
        let path = "new.txt";
        let dm = helper::init(path);

        assert_eq!(0, dm.next_page_id);

        helper::cleanup(path).unwrap();
    }

    #[test]
    // allocate_page で PageId を返すこと、
    // next_page_id が increment されること
    fn allocate_page() {
        let path = "allocate_page.txt";
        let mut dm = helper::init(path);

        assert_eq!(0, dm.next_page_id);
        let next_page_id = dm.allocate_page();
        assert_eq!(PageId(0), next_page_id);
        assert_eq!(1, dm.next_page_id);

        let next_page_id = dm.allocate_page();
        assert_eq!(PageId(1), next_page_id);
        assert_eq!(2, dm.next_page_id);

        helper::cleanup(path).unwrap();
    }

    #[test]
    fn write_page_data() {
        let path = "write_page_data.txt";
        let mut dm = helper::init(path);
        let page_id = dm.allocate_page();
        let page_id_2 = page_id.clone();

        let mut data = Vec::with_capacity(PAGE_SIZE);
        data.extend_from_slice(b"hello");
        data.resize(PAGE_SIZE, 0);
        dm.write_page_data(page_id, &data).unwrap();

        let mut read = vec![0; PAGE_SIZE];
        dm.read_page_data(page_id_2, &mut read).unwrap();

        assert_eq!(data, read);

        helper::cleanup(path).unwrap();
    }

    #[test]
    fn read_page_data() {
        let path = "read_page_data.txt";
        let mut dm = helper::init(path);
        let page_id = dm.allocate_page();
        let page_id_2 = page_id.clone();

        let mut data = Vec::with_capacity(PAGE_SIZE);
        data.extend_from_slice(b"world");
        data.resize(PAGE_SIZE, 0);
        dm.write_page_data(page_id, &data).unwrap();

        let mut read = vec![0; PAGE_SIZE];
        dm.read_page_data(page_id_2, &mut read).unwrap();

        assert_eq!(data, read);

        helper::cleanup(path).unwrap();
    }
}
