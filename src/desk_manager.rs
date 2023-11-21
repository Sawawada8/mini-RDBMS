use std::{fs::File, io};

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

impl DeskManager {
    // pub fn new(data_file: File) -> io::Result<Self> {
    //     DeskManager {}
    // }
}
