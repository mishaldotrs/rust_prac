1 trait — Notifier
    ↓
Concept 1 → Trait basics        trait Notifier { fn send() }
Concept 2 → Default impl        fn send_with_prefix() { ... }
Concept 3 → Trait impl          impl Notifier for Email/Sms/Push
Concept 4 → Trait bound         fn notify_user<T: Notifier>()
Concept 5 → dyn Trait           Box<dyn Notifier>
Concept 6 → impl Trait          -> impl Notifier
