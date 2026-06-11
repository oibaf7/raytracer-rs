use crate::camera::{Camera, CameraParams};
use crate::hittable::HittableList;
use crate::tiles::{ResultTile, Tile};
use crate::vector::Vec3;
use std::f64::consts::PI;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::{available_parallelism, JoinHandle, Thread};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    handle: JoinHandle<()>,
}

impl Worker {
    pub fn new(
        id: usize,
        sender: Sender<ResultTile>,
        tiles: Arc<Vec<Tile>>,
        counter: Arc<AtomicUsize>,
        params: Arc<CameraParams>,
        list: Arc<HittableList>,
    ) -> Self {
        let handle = thread::spawn(move || {
            loop {
                let n = counter.fetch_add(1, Ordering::Relaxed);
                if n >= tiles.len() {
                    break;
                }
                let tile = &tiles[n];
                let colors = Camera::render_chunk(tile, &list, &params);
                sender.send(ResultTile::new(n, colors)).unwrap();
            }
        });

        Self { id, handle }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(
        sender: Sender<ResultTile>,
        tiles: Vec<Tile>,
        params: CameraParams,
        list: HittableList,
    ) -> Self {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut workers = Vec::new();
        let tiles = Arc::new(tiles);
        let params = Arc::new(params);
        let list = Arc::new(list);
        let cores = available_parallelism().unwrap().get();
        for i in 0..cores {
            let worker = Worker::new(
                i,
                sender.clone(),
                Arc::clone(&tiles),
                Arc::clone(&counter),
                Arc::clone(&params),
                Arc::clone(&list),
            );
            workers.push(worker);
        }

        Self {
            workers,
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        for worker in self.workers.drain(..) {
            worker.handle.join().unwrap();
        }
    }
}
