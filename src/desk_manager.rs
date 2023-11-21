use std::{fs::File, io::{self, SeekFrom, Write, Read}};
use std::io::Seek;

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
pub struct DeskManager {
    heap_file: File,
    next_page_id: u64,
}
#[derive(PartialEq)]
#[derive(Debug)]
pub struct PageId(pub u64);
// new type pattern というらしい

const PAGE_SIZE: usize = 4096;

impl DeskManager {
    pub fn new(heap_file: File) -> io::Result<Self> {
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;
        Ok( Self {
            heap_file,
            next_page_id,
        })
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

    pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) ->io::Result<()> {
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
        use super::*;

        pub fn init() -> io::Result<DeskManager> {
            let heap_file = File::create("foo.txt")?;
            let dm = DeskManager::new(heap_file)?;
            Ok(dm)
        }
    }

    #[test]
    fn new() -> io::Result<()> {
        let dm = helper::init()?;

        assert_eq!(0, dm.next_page_id);
        Ok(())
    }

    #[test]
    // allocate_page で PageId を返すこと、
    // next_page_id が increment されること
    fn allocate_page() -> io::Result<()> {
        let mut dm = helper::init()?;

        assert_eq!(0, dm.next_page_id);
        let next_page_id = dm.allocate_page();
        assert_eq!(PageId(0), next_page_id);
        assert_eq!(1, dm.next_page_id);

        let next_page_id = dm.allocate_page();
        assert_eq!(PageId(1), next_page_id);
        assert_eq!(2, dm.next_page_id);

        Ok(())
    }
}
