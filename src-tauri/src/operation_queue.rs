use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperationPriority {
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    Copy,
    Move,
    Delete,
    Archive,
    Extract,
}

#[derive(Debug, Clone, Serialize)]
pub struct OperationInfo {
    pub id: String,
    pub op_type: OperationType,
    pub priority: OperationPriority,
    pub sources: Vec<String>,
    pub destination: Option<String>,
    pub progress: f64,
    pub current_file: String,
    pub done: usize,
    pub total: usize,
    pub paused: bool,
    pub status: String,
}

pub struct Operation {
    pub info: OperationInfo,
    pub cancel_flag: Arc<AtomicBool>,
    pub pause_flag: Arc<AtomicBool>,
}

pub struct OperationQueue {
    pub operations: Arc<Mutex<Vec<Operation>>>,
    _max_concurrent: usize,
}

#[allow(dead_code)]
impl OperationQueue {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            operations: Arc::new(Mutex::new(Vec::new())),
            _max_concurrent: max_concurrent,
        }
    }

    pub fn create_operation(
        &self,
        op_type: OperationType,
        priority: OperationPriority,
        sources: Vec<String>,
        destination: Option<String>,
    ) -> String {
        let id = Uuid::new_v4().to_string();
        let op = Operation {
            info: OperationInfo {
                id: id.clone(),
                op_type,
                priority,
                sources,
                destination,
                progress: 0.0,
                current_file: String::new(),
                done: 0,
                total: 0,
                paused: false,
                status: "queued".into(),
            },
            cancel_flag: Arc::new(AtomicBool::new(false)),
            pause_flag: Arc::new(AtomicBool::new(false)),
        };

        if let Ok(mut ops) = self.operations.lock() {
            ops.push(op);
        }

        id
    }

    pub fn list_operations(&self) -> Vec<OperationInfo> {
        if let Ok(ops) = self.operations.lock() {
            ops.iter().map(|op| op.info.clone()).collect()
        } else {
            Vec::new()
        }
    }

    pub fn pause_operation(&self, id: &str) -> Result<(), String> {
        if let Ok(ops) = self.operations.lock() {
            if let Some(op) = ops.iter().find(|o| o.info.id == id) {
                op.pause_flag.store(true, Ordering::Relaxed);
                return Ok(());
            }
        }
        Err("Operation not found".into())
    }

    pub fn resume_operation(&self, id: &str) -> Result<(), String> {
        if let Ok(ops) = self.operations.lock() {
            if let Some(op) = ops.iter().find(|o| o.info.id == id) {
                op.pause_flag.store(false, Ordering::Relaxed);
                return Ok(());
            }
        }
        Err("Operation not found".into())
    }

    pub fn cancel_operation(&self, id: &str) -> Result<(), String> {
        if let Ok(ops) = self.operations.lock() {
            if let Some(op) = ops.iter().find(|o| o.info.id == id) {
                op.cancel_flag.store(true, Ordering::Relaxed);
                return Ok(());
            }
        }
        Err("Operation not found".into())
    }

    pub fn remove_operation(&self, id: &str) {
        if let Ok(mut ops) = self.operations.lock() {
            ops.retain(|o| o.info.id != id);
        }
    }

    pub fn update_progress(&self, id: &str, done: usize, total: usize, current_file: &str) {
        if let Ok(mut ops) = self.operations.lock() {
            if let Some(op) = ops.iter_mut().find(|o| o.info.id == id) {
                op.info.done = done;
                op.info.total = total;
                op.info.current_file = current_file.to_string();
                op.info.progress = if total > 0 { done as f64 / total as f64 } else { 0.0 };
                op.info.status = "running".into();
            }
        }
    }
}

lazy_static::lazy_static! {
    pub static ref QUEUE: OperationQueue = OperationQueue::new(2);
}
