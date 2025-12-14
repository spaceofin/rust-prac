INSERT INTO articles (id, title, body, draft, publish_at, visit_count) VALUES
(1, 'Getting Started with Rust',
 'An introduction to Rust focusing on ownership, borrowing, and why it is a safe systems programming language.',
 0, '2025-12-01 10:00:00', 42),

(2, 'Understanding Ownership',
 'This article explains Rust’s ownership model with simple examples and common pitfalls beginners face.',
 0, '2025-12-02 11:30:00', 58),

(3, 'Borrowing and Lifetimes',
 'Borrowing rules and lifetimes can feel complex, but they exist to prevent data races and invalid memory access.',
 1, '2025-12-03 09:15:00', 12),

(4, 'Why Rust Has No Garbage Collector',
 'Rust avoids a garbage collector by enforcing strict compile-time checks, leading to predictable performance.',
 0, '2025-12-04 14:00:00', 67),

(5, 'Enums and Pattern Matching',
 'Enums combined with pattern matching allow expressive and safe control flow in Rust applications.',
 0, '2025-12-05 16:45:00', 33),

(6, 'Struct Design Best Practices',
 'Learn how to design structs that are easy to use, extend, and maintain over time.',
 1, '2025-12-06 13:20:00', 9),

(7, 'Error Handling with Result',
 'Rust encourages explicit error handling using the Result type instead of exceptions.',
 0, '2025-12-07 18:10:00', 51),

(8, 'Option vs Result',
 'This article compares Option and Result, explaining when each type should be used.',
 0, '2025-12-08 08:40:00', 29),

(9, 'Traits Explained',
 'Traits define shared behavior in Rust and are similar to interfaces in other languages.',
 1, '2025-12-09 15:00:00', 14),

(10, 'Generic Programming in Rust',
 'Generics allow writing flexible and reusable code while maintaining strong type safety.',
 0, '2025-12-10 17:35:00', 61),

(11, 'Lifetimes in Practice',
 'Practical lifetime examples help clarify how Rust ensures references remain valid.',
 0, '2025-12-11 10:10:00', 47),

(12, 'Interior Mutability Patterns',
 'Interior mutability enables mutation through immutable references using types like RefCell.',
 0, '2025-12-12 12:25:00', 38),

(13, 'Smart Pointers Overview',
 'Rust provides smart pointers such as Box, Rc, and Arc for different ownership scenarios.',
 1, '2025-12-13 14:50:00', 11),

(14, 'Understanding Rc and Arc',
 'Rc and Arc enable shared ownership, but each comes with different trade-offs.',
 0, '2025-12-14 09:30:00', 44),

(15, 'Concurrency without Fear',
 'Rust’s type system prevents data races at compile time, making concurrency safer.',
 0, '2025-12-15 11:55:00', 72),

(16, 'Async Programming Basics',
 'An introduction to async and await in Rust, including common async runtimes.',
 1, '2025-12-16 16:05:00', 17),

(17, 'Using Diesel with SQLite',
 'This article shows how to use Diesel ORM with SQLite for type-safe database access.',
 0, '2025-12-17 09:20:00', 56),

(18, 'Writing Migrations in Diesel',
 'Diesel migrations help keep database schemas in sync with application code.',
 0, '2025-12-18 14:15:00', 34),

(19, 'Testing Rust Applications',
 'Learn how to write unit tests and integration tests in Rust projects.',
 0, '2025-12-19 13:40:00', 49),

(20, 'When to Use Unsafe Rust',
 'Unsafe Rust allows low-level control but should be used carefully and sparingly.',
 0, '2025-12-20 10:30:00', 63);
