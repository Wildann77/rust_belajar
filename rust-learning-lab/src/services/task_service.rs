// Modul Task Service: Business Logic Pengelolaan Task
// Lokasi: rust-learning-lab/src/services/task_service.rs

use crate::auth::Claims;
use crate::models::{Priority, Task, TaskStatus};

#[derive(Debug)]
pub struct TaskService {
    tasks: Vec<Task>,
    pub(crate) app_name: String, // pub(crate): field internal crate
    next_id: u64,
}

impl TaskService {
    pub fn new(app_name: &str) -> Self {
        Self {
            tasks: Vec::new(),
            app_name: app_name.to_string(),
            next_id: 1,
        }
    }

    /// Membuat task baru dengan validasi otentikasi
    pub fn create_task(
        &mut self,
        title: &str,
        priority: Priority,
        claims: &Claims,
    ) -> Result<u64, &'static str> {
        // Validasi menggunakan method pub(crate) dari Claims
        if !claims.is_session_active() {
            return Err("Sesi user tidak aktif");
        }

        let id = self.next_id;
        let mut task = Task::new(id, title, priority);

        // Menggunakan helper pub(crate) dari model Task
        task.set_internal_notes(&format!(
            "Dibuat oleh user '{}' pada app '{}'",
            claims.sub, self.app_name
        ));

        self.tasks.push(task);
        self.next_id += 1;
        Ok(id)
    }

    pub fn list_tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn get_task(&self, id: u64) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }

    pub fn get_task_mut(&mut self, id: u64) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    /// Memajukan status task dengan validasi hak akses
    pub fn advance_task(&mut self, id: u64, claims: &Claims) -> Result<TaskStatus, &'static str> {
        let task = self
            .get_task_mut(id)
            .ok_or("Task dengan ID tersebut tidak ditemukan")?;

        // Jika task berstatus Critical, hanya admin yang boleh memajukan
        if task.priority == Priority::Critical && !claims.is_admin() {
            return Err("Hanya admin yang berhak mengubah status task berkategori Critical");
        }

        let new_status = task.advance_status()?;
        task.set_internal_notes(&format!(
            "Status dimajukan ke {:?} oleh '{}'",
            new_status, claims.sub
        ));

        Ok(new_status)
    }

    pub fn filter_by_priority(&self, priority: Priority) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.priority == priority)
            .collect()
    }

    // pub(crate) method: hanya visible di dalam crate ini
    pub(crate) fn internal_task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn total_tasks(&self) -> usize {
        self.internal_task_count()
    }

    pub fn get_task_notes(&self, id: u64) -> Option<&str> {
        self.get_task(id).map(|t| t.internal_notes())
    }
}
