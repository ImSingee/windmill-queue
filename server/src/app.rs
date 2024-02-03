use tokio::sync::mpsc;
use anyhow::Result;
use crate::task::Task;

pub struct App {
    task_tx: mpsc::Sender<Task>,
    task_rx: mpsc::Receiver<Task>,
}

pub type TaskSender = mpsc::Sender<Task>;

impl App {
    pub fn new() -> Self {
        let (tx, mut rx) = mpsc::channel::<Task>(32);

        Self {
            task_tx: tx,
            task_rx: rx,
        }
    }

    pub fn task_sender(&self) -> TaskSender {
        self.task_tx.clone()
    }

    pub async fn run(&mut self) -> Result<()> {
        while let Some(event) = self.task_rx.recv().await {
            match event {
                Task::Demo(msg) => {
                    println!("New Demo Task: {}", msg);
                }
            }
        }

        Ok(())
    }
}