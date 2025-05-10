// Импортируем необходимые библиотеки
use std::io;
use std::io::Write; // Для использования метода flush()
use std::time::{SystemTime, UNIX_EPOCH};

// Определяем структуру для хранения информации о задаче
struct Task {
    id: u64,           // Уникальный идентификатор задачи
    title: String,     // Заголовок задачи
    completed: bool,   // Статус выполнения
}

// Реализация методов для структуры Task
impl Task {
    // Конструктор для создания новой задачи
    fn new(id: u64, title: String) -> Task {
        Task {
            id,
            title,
            completed: false, // По умолчанию задача не выполнена
        }
    }

    // Метод для отображения задачи
    fn display(&self) {
        let status = if self.completed { "[✓]" } else { "[ ]" };
        println!("{} {} - {}", self.id, status, self.title);
    }

    // Метод для переключения статуса задачи
    fn toggle_status(&mut self) {
        self.completed = !self.completed;
    }
}

// Структура для управления списком задач
struct TaskManager {
    tasks: Vec<Task>, // Вектор для хранения задач
}

// Реализация методов для структуры TaskManager
impl TaskManager {
    // Конструктор для создания нового менеджера задач
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(), // Создаем пустой вектор
        }
    }

    // Метод для добавления новой задачи
    fn add_task(&mut self, title: String) {
        // Генерируем уникальный ID на основе времени
        let id = generate_id();
        let task = Task::new(id, title);
        self.tasks.push(task);
        println!("Задача добавлена с ID: {}", id);
    }

    // Метод для отображения всех задач
    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("Список задач пуст.");
            return;
        }

        println!("\n--- Список задач ---");
        for task in &self.tasks {
            task.display();
        }
        println!("-------------------");
    }

    // Метод для переключения статуса задачи по ID
    fn toggle_task(&mut self, id: u64) -> bool {
        for task in &mut self.tasks {
            if task.id == id {
                task.toggle_status();
                return true;
            }
        }
        false // Задача с таким ID не найдена
    }

    // Метод для удаления задачи по ID
    fn remove_task(&mut self, id: u64) -> bool {
        let initial_len = self.tasks.len();
        // Фильтруем вектор, оставляя только задачи с другими ID
        self.tasks.retain(|task| task.id != id);
        
        // Проверяем, изменилась ли длина вектора
        self.tasks.len() < initial_len
    }
}

// Функция для генерации уникального ID
fn generate_id() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(), // Используем количество секунд с начала эпохи Unix
        Err(_) => panic!("Системное время установлено до начала эпохи Unix!"),
    }
}

// Функция для чтения строки ввода с консоли
fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка при чтении строки");
    input.trim().to_string()
}

// Основная функция
fn main() {
    println!("Менеджер задач на Rust");
    println!("----------------------");

    // Создаем новый менеджер задач
    let mut task_manager = TaskManager::new();

    // Основной цикл программы
    loop {
        println!("\nВыберите действие:");
        println!("1. Добавить задачу");
        println!("2. Показать список задач");
        println!("3. Отметить задачу как выполненную/невыполненную");
        println!("4. Удалить задачу");
        println!("5. Выйти");

        print!("Ваш выбор: ");
        // Сбрасываем буфер вывода, чтобы сразу отобразить приглашение
        io::stdout().flush().expect("Ошибка при очистке буфера");

        // Считываем выбор пользователя
        let choice = read_input();

        // Обрабатываем выбор пользователя
        match choice.as_str() {
            "1" => {
                print!("Введите название задачи: ");
                io::stdout().flush().expect("Ошибка при очистке буфера");
                let title = read_input();
                task_manager.add_task(title);
            }
            "2" => {
                task_manager.list_tasks();
            }
            "3" => {
                print!("Введите ID задачи: ");
                io::stdout().flush().expect("Ошибка при очистке буфера");
                let id_str = read_input();
                // Преобразуем строку в u64
                match id_str.parse::<u64>() {
                    Ok(id) => {
                        if task_manager.toggle_task(id) {
                            println!("Статус задачи с ID {} изменен.", id);
                        } else {
                            println!("Задача с ID {} не найдена.", id);
                        }
                    }
                    Err(_) => println!("Некорректный ID. Пожалуйста, введите число."),
                }
            }
            "4" => {
                print!("Введите ID задачи для удаления: ");
                io::stdout().flush().expect("Ошибка при очистке буфера");
                let id_str = read_input();
                // Преобразуем строку в u64
                match id_str.parse::<u64>() {
                    Ok(id) => {
                        if task_manager.remove_task(id) {
                            println!("Задача с ID {} удалена.", id);
                        } else {
                            println!("Задача с ID {} не найдена.", id);
                        }
                    }
                    Err(_) => println!("Некорректный ID. Пожалуйста, введите число."),
                }
            }
            "5" => {
                println!("Выход из программы. До свидания!");
                break;
            }
            _ => println!("Некорректный выбор. Пожалуйста, выберите число от 1 до 5."),
        }
    }
}