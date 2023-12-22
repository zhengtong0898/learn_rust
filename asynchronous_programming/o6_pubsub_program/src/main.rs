use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Student {
    name: String,
    age: i64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Teacher {
    name: String,
    category: String,
}

#[allow(dead_code)]
struct ClassRoom {
    students: HashSet<Arc<Student>>,
    teachers: Vec<Teacher>,
}

#[allow(dead_code)]
enum ClassRoomAction {
    AddStudent(Arc<Student>, Sender<anyhow::Result<()>>),
    IsStudentExist(String, Sender<anyhow::Result<bool>>),
}

#[allow(dead_code)]
struct ClassRoomManager {
    sender: Sender<ClassRoomAction>,
    receiver: Mutex<Receiver<ClassRoomAction>>,
}

impl ClassRoomManager {
    fn new() -> Self {
        let (sender, receiver) = mpsc::channel(32);
        Self {
            sender,
            receiver: Mutex::new(receiver),
        }
    }

    // 服务运行: 读写都在这里发生.
    async fn run_serve(&self) {
        let mut class_room = ClassRoom {
            students: HashSet::new(),
            teachers: Vec::new(),
        };

        let mut receiver_guard = self.receiver.lock().await;
        while let Some(action) = receiver_guard.recv().await {
            match action {
                ClassRoomAction::AddStudent(student, tmp_sender) => {
                    class_room.students.insert(student);
                    tmp_sender.send(Ok(())).await.unwrap();
                }
                ClassRoomAction::IsStudentExist(name, tmp_sender) => {
                    let exists = class_room.students.iter().any(|s| s.name == name);
                    tmp_sender.send(Ok(exists)).await.unwrap();
                }
            }
        }
    }

    // 行为发起: 请求添加用户
    async fn add_user(&self, student: Arc<Student>) -> anyhow::Result<()> {
        let (tmp_sender, mut tmp_receiver) = mpsc::channel(1);

        self.sender
            .send(ClassRoomAction::AddStudent(student.clone(), tmp_sender))
            .await
            .unwrap();

        match tmp_receiver.recv().await {
            Some(v) => {
                if let Err(e) = v {
                    return Err(e);
                }
            }
            None => {
                return Err(anyhow::anyhow!("tmp_receiver: channel has been closed."));
            }
        }
        Ok(())
    }

    // 行为发起: 请求查询用户是否存在
    async fn is_user_exist(&self, name: &str) -> anyhow::Result<bool> {
        let (tmp_sender, mut tmp_receiver) = mpsc::channel(1);

        self.sender
            .send(ClassRoomAction::IsStudentExist(name.into(), tmp_sender))
            .await
            .unwrap();

        match tmp_receiver.recv().await {
            Some(v) => v,
            None => Err(anyhow::anyhow!("tmp_receiver: channel has been closed.")),
        }
    }
}

#[tokio::main]
async fn main() {
    let class_room_manager = Arc::new(ClassRoomManager::new());

    // 启动独立的服务, 用于接收读写请求.
    let manager_clone = class_room_manager.clone();
    let join_handler = tokio::spawn(async move {
        manager_clone.run_serve().await;
    });

    // 业务场景-1: 创建一个 user 对象
    {
        let student = Arc::new(Student {
            name: "Alice".into(),
            age: 20,
        });

        // 将 user 对象添加至 classroom
        if let Err(e) = class_room_manager.add_user(student.clone()).await {
            println!("Error: 添加用户报错: {e:?}");
        } else {
            println!("用户({})添加成功!", student.name);
        }
    }

    // 业务场景-2: 查询用户是否存在
    {
        let student_name = "Alice";
        match class_room_manager.is_user_exist(student_name).await {
            Ok(v) => {
                if v {
                    println!("用户存在.");
                } else {
                    println!("用户不存在.");
                }
            }
            Err(e) => {
                println!("Error: 查询用户{}是否存在时报错: {e:?}", student_name);
            }
        }
    }

    // 检查线程是否出现异常情况.
    if let Err(e) = join_handler.await {
        println!("Error: 启动独立的服务失败. {e:?}");
    }
}
