pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        pub sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // 1. RefCell 要求编译期跳过借用规则检查.
            // 2. RefCell 要求程序在运行期进行借用规则检查.
            // 3. self.sent_messages.borrow_mut() 是对 self.sent_messages 进行&mut借用,
            //    而不是对self进行&mut借用, 所以没有违背借用原则.
            // 4. 注意事项:
            let mut ss = self.sent_messages.borrow_mut();
            ss.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        // 如果这里增加这两行代码, 就会在运行时报错.
        // 1. 由于这个代码不是在编译期进行借用规则检查, 不会运用Non-Lexical-Lifetimes来解决"变量即时回收"问题.
        // 2. ss变量在运行期做借用规则检查, 程序会认为ss变量的生命周期与当前函数一行长.
        // 3. limit_tracker.set_value(80), 会触发mock_mesager.send方法, 造成了第二次 borrow_mut, 最终panic崩溃.
        // let mut ss = mock_messenger.sent_messages.borrow_mut();
        // ss.push(String::from("ssss"));

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 2);
    }
}
