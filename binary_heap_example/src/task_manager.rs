use core::time;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Debug)]
struct Task {
    name: String,
    time: time::Duration,
    description: String,
}

/**
 * The PartialOrd trait is used to compare two values who could be uncomparable (not tatally comparable) e.g NaN
 * Every Ord type should implement PartialOrd trait as fallback.
 * If PartialOrd is not implemented, the Ord trait can panic. e.g: 2 is not comparable to NaN (in reality compiler will not allow this to happen, but it's a good example to understand the concept of PartialOrd and Ord traits)
 * [Compiler doesn't allow to implement Ord trait without implementing PartialOrd trait]
 */
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // self.time.cmp(&other.time) // This will make the BinaryHeap a max-heap e.g: pop() will return the highest time task
        // Reverse the ordering to make the BinaryHeap a min-heap e.g: other.time.cmp(&self.time) or self.time.cmp(&other.time).reverse()
        self.time.cmp(&other.time).reverse()
    } 
}

struct TaskManager {
    tasks: BinaryHeap<Task>,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager {
            tasks: BinaryHeap::new(),
        }
    }

    fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn pop(&mut self) -> Option<Task> {
        self.tasks.pop()
    }

    fn peek(&self) -> Option<&Task> {
        self.tasks.peek()
    }

    fn len(&self) -> usize {
        self.tasks.len()
    }

    fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_manager() {
        let mut task_manager = TaskManager::new();
        assert_eq!(task_manager.len(), 0);
        assert_eq!(task_manager.is_empty(), true);

        let task1 = Task {
            name: "Task 1".to_string(),
            time: time::Duration::from_secs(10),
            description: "Task 1 description".to_string(),
        };
        let task2 = Task {
            name: "Task 2".to_string(),
            time: time::Duration::from_secs(5),
            description: "Task 2 description".to_string(),
        };
        let task3 = Task {
            name: "Task 3".to_string(),
            time: time::Duration::from_secs(15),
            description: "Task 3 description".to_string(),
        };

        task_manager.add(task1);
        task_manager.add(task2);
        task_manager.add(task3);

        assert_eq!(task_manager.len(), 3);
        assert_eq!(task_manager.is_empty(), false);

        let task = task_manager.pop();
        assert_eq!(task_manager.len(), 2);
        assert_eq!(task_manager.is_empty(), false);
        assert_eq!(task.unwrap().time, time::Duration::from_secs(5));

        let task = task_manager.peek();
        assert_eq!(task_manager.len(), 2);
        assert_eq!(task_manager.is_empty(), false);
        assert_eq!(task.unwrap().time, time::Duration::from_secs(10));
    }
}