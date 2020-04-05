
fn open_mmap(full_path: &Path) -> result::Result<Option<Mmap>, OpenReadError> {
    let file = File::open(full_path).map_err(|e| {
        if e.kind() == io::ErrorKind::NotFound {
            OpenReadError::FileDoesNotExist(full_path.to_owned())
        } else {
            OpenReadError::IOError(IOError::with_path(full_path.to_owned(),e))
        }
    })?;
    let meta_data = file
        .metadata()
        .map_err(|e| IOError::with_path(full_path.to_owned(), e))?;
    if meta_data.len() == 0 {
        // if the file size is 0, it will not be possible
        // to mmap the file, so we return None
        // instead.
        return Ok(None);
    }
    unsafe {
        memmap::Mmap::map(&file)
            .map(Some)
            .map_err(|e| From::from(IOError::with_path(full_path.to_owned(), e)))
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CacheCounters {
    // Number of time the cache prevents to call `mmap`
    pub hit: usize,
    // Number of time tantivy had to call `mmap`
    // as no entry was in the cache.
    pub miss: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CacheInfo {
    pub counters: CacheCounters,
    pub mmapped: Vec<PathBuf>,
}
struct MmapCache {
    counters: CacheCounters,
    cache: HashMap<PathBuf, Weak<BoxedData>>,
}

impl MmapCache {
    fn get_info(&self) -> CacheInfo {
        let paths: Vec<PathBuf> = self.cache.keys().cloned().collect();
        CacheInfo {
            counters: self.counters.clone(),
            mmapped: paths,
        }
    }

    fn remove_weak_ref(&mut self) {
        let keys_to_remove: Vec<PathBuf> = self
            .cache
            .iter()
            .filter(|(_, mmap_weakref)| mmap_weakref.upgrade().is_none())
            .map(|(key, _)| key.clone())
            .collect();
        for key in keys_to_remove {
            self.cache.remove(&key);
        }
    }

    // Returns None if the file exists but as a len of 0 (and hence is not mmappable).
    fn get_mmap(&mut self, full_path: &Path) -> Result<Option<Arc<BoxedData>>, OpenReadError> {
        if let Some(mmap_weak) = self.cache.get(full_path) {
            if let Some(mmap_arc) = mmap_weak.upgrade() {
                self.counters.hit += 1;
                return Ok(Some(mmap_arc));
            }
        }
        self.cache.remove(full_path);
        self.counters.miss += 1;
        let mmap_opt = open_mmap(full_path)?;
        Ok(mmap_opt.map(|mmap| {
            let mmap_arc: Arc<BoxedData> = Arc::new(Box::new(mmap));
            let mmap_weak = Arc::downgrade(&mmap_arc);
            self.cache.insert(full_path.to_owned(), mmap_weak);
            mmap_arc
        }))
    }
}