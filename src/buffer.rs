
/// disk の遅さを隠蔽するために、
/// disk manager に buffer pool manager を被せて使う
/// ディスクアクセスはメモリアクセスに比べて非常に遅い、ページの読み込みの度にディスクアクセスはパフォーマンスがやばい
///
/// バッファープールマネージャ
/// ページの内容をメモリにキャッシュしする
/// ディスクアクセスせずにメモリ上から返す
/// ２回目以降メモリの速度で返せるようになる
///
/// ファイルシステムにもキャッシュの機能があるがそれに頼らず RDBMS 専用の機構を利用することでより賢くキャッシュ管理する
///
/// バッファープール
/// ページのデータをキャッシュするメモリ上の場所
/// （特定の処理のために割り当てられたメモリは、バッファーと言われる？）
///
/// ページid とバッファーid の対応テーブルを管理している
///
/// バッファープールの内部構造（ミニRDBMSの場合）
/// - バッファープール
///   - フレーム
///     - バッファ
///       - ページ
///   - フレーム ...
///   - フレーム ...
///
/// バッファ
/// ページ + page id + is_dirty
///
/// フレーム
/// バッファ + usage_count
///
/// ※ is_dirty, usage_count でバッファープールを管理していく
///
/// ページテーブル
/// バッファとページIDの対応を管理
///
///
/// Rc
/// 対象データへの参照の数を実行時に追跡してカウントする
/// ０になったらメモリ領域を開放する
///
/// RefCell
/// 複雑なデータ構造の競合を実行時に検査してくれる
///
/// Cell
/// 読み取り専用の値の中に書き込み専用な値を作るために使う
///
///
/// どのバッファを捨てるか決めるアルゴリズム
/// 再利用しなさそうなものを捨てる
///
/// Clock-sweep
/// postgreSql でも採用されているアルゴリズム
/// 性能の割に実装が簡単
/// BufferPool::evict で実装する

mod buffer;

pub type Page = [u8; PAGE_SIZE];

pub struct Buffer {
    pub page_id: PageId,
    pub page: RefCell<Page>,
    pub is_dirty: Cell<bool>,
}

pub struct Frame {
    usage_count: u64,
    buffer: Rc<Buffer>,
}

pub struct BufferPool {
    buffers: Vec<Frame>,
    next_victim_id: BufferId,
}

pub struct BufferPoolManager {
    disk: DiskManager,
    pool: BufferPool,
    page_table: HashMap<PageId, BufferId>,
}

impl BufferPool {
    // すべてのバッファを巡回する
    fn evict(&mut self) -> Option<BufferId> {
        let pool_size = self.size();
        let mut consective_pinned = 0;
        let victim_id = loop {
            let next_victim_id = self.next_victim_id;
            let frame = &mut self[next_victim_id];

            // 利用回数０
            if frame.usage_count == 0 {
                break self.next_victim_id;
            }
        };
        if Rc::get_mut(&mut frame.buffer).is_some() {
            frame.usage_count -= 1;
            consective_pinned = 0;
        } else {
            consective_pinned += 1;
            // すべてのブッファが貸出中の為 none
            if consective_pinned >= pool_size {
                return None;
            }
        }
        Some(victim_id)
    }

    fn increment_id(&self, buffer_id: BufferId) -> BufferId {
        BufferId((buffer_id.0 + 1) % self.size())
    }
}

impl BufferPoolManager {
    pub fn fetch_page(&mut self, page_id: PageId) -> Result<Rc<Buffer>, Error> {
        if let Some(&buffer_id) = self.page_table.get(&page_id) {
            let frame = &mut self.pool[buffer_id];
            frame.usage_count += 1;
            return Ok(frame.buffer.clone());
        }

        let buffer_id = self.pool.evict().ok_or(Error::NoFreeBuffer)?;
        let frame = &mut self.pool[buffer_id];
        let evict_page_id = frame.buffer.page_id;
        {
            let buffer = Rc::get_mut(&mut frame.buffer).unwrap();
            if buffer.is_dirty.get() {
                self.disk.write_page_data(evict_page_id, buffer.page.get_mut())?;
            }
            buffer.page_id = page_id;
            buffer.is_dirty.set(false);
            self.dis.read_page_data(page_id, buffer.page.get_mut())?;
            frame.usage_count = 1;
        }
        let page = Rc::clone(&frame.buffer);
        self.page_table.remove(&evict_page_id);
        self.page_table.insert(page_id, buffer_id);
    }
}
