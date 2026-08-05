// 1 trait — Notifier
//     ↓
// Concept 1 → Trait basics        trait Notifier { fn send() }
// Concept 2 → Default impl        fn send_with_prefix() { ... }
// Concept 3 → Trait impl          impl Notifier for Email/Sms/Push
// Concept 4 → Trait bound         fn notify_user<T: Notifier>()
// Concept 5 → dyn Trait           Box<dyn Notifier>
// Concept 6 → impl Trait          -> impl Notifier


trait Notifier {
    fn send(&self, to: &str, message: &str);

    fn send_with_prefix(&self, to: &str, message: &str) {
        let full_msg = format!("[ALERT] {}", message);
        self.send(to, &full_msg);
    }
}

struct EmailNotifier {
    smtp_server: String,
}

struct SmsNotifier {
    api_key: String,
}

struct PushNotifier {
    app_id: String,
}

impl Notifier for EmailNotifier {
    fn send(&self, to: &str, message: &str) {
        println!(
            "[EMAIL] via {} → to: {} | msg: {}",
            self.smtp_server, to, message
        );
    }
}

impl Notifier for SmsNotifier {
    fn send(&self, to: &str, message: &str) {
        println!("[SMS] via {} → to: {} | msg: {}", self.api_key, to, message);
    }
}

impl Notifier for PushNotifier {
    fn send(&self, to: &str, message: &str) {
        println!("[PUSH] via {} → to: {} | msg: {}", self.app_id, to, message);
    }
}

fn notify_user<T: Notifier>(notifier: T, user: &str, msg: &str) {
    notifier.send(user, msg);
}

fn get_notifier(service: &str) -> Box<dyn Notifier> {
    match service {
        "email" => Box::new(EmailNotifier {
            smtp_server: String::from("smtp.gmail.com"),
        }),
        "sms" => Box::new(SmsNotifier {
            api_key: String::from("twilio_key_123"),
        }),
        _ => Box::new(PushNotifier {
            app_id: String::from("firebase_app_1"),
        }),
    }
}

fn default_notifier() -> impl Notifier {
    EmailNotifier {
        smtp_server: String::from("smtp.default.com"),
    }
}

fn main() {
    let email = EmailNotifier {
        smtp_server: String::from("smtp.gmail.com"),
    };
    let sms = SmsNotifier {
        api_key: String::from("twilio_key_123"),
    };
    let push = PushNotifier {
        app_id: String::from("firebase_app_1"),
    };

    println!("=== Direct Use ===");
    email.send("mishal@gmail.com", "welcome!");
    sms.send("+91-9999999999", "your otp is 1234");
    push.send("device_token_xyz", "you have a new message");

    println!("\n=== Default impl ===");
    email.send_with_prefix("mishal@gmail.com", "your account was logged in");

    println!("\n=== Generic Function ===");
    notify_user(
        SmsNotifier {
            api_key: String::from("key_abc"),
        },
        "+91-8888888888",
        "hello via generic!",
    );

    println!("\n=== dyn Trait ===");
    let services = ["email", "sms", "push"];
    for service in services {
        let notifier = get_notifier(service);
        notifier.send("mishal", &format!("message from {}", service));
    }

    println!("\n=== impl Trait ===");
    let n = default_notifier();
    n.send("mishal@gmail.com", "default notifier message");
}
